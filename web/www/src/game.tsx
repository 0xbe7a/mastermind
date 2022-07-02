import React, { useEffect, useRef } from 'react';
import { Colors } from './colors';
import { Menu } from './menu';
import { Board } from './board';
import { useState } from 'react';
import { useReducer } from 'react';
import type { ConcreteSolver, SolverType } from '.';
import { Grid } from 'react-loader-spinner';


const ColorNames = ['red', 'green', 'blue', 'orange', 'purple', 'rose', 'black', 'darkBlue'];

type RowStore = {
  finished: boolean
  pegCount: number,
  colorCount: number,
  currentRow: string[]
  currentHint: [number, number]
  previousRows: string[][],
  previousHints: [number, number][],
}

function initGame([pegCount, colorCount]: [number, number]): RowStore {
  return {
    finished: false,
    pegCount: pegCount,
    colorCount: colorCount,
    previousRows: [],
    previousHints: [],
    currentHint: [0, 0],
    currentRow: Array(pegCount).fill('')
  }

}

type RowAction = { type: 'reset', pegCount: number, colorCount: number } | { type: 'submit' } | { type: 'peg', idx: number, color: string } | { type: 'hint', idx: number, count: number } | { type: 'solution', colors: string[] }

function gameReducer(state: RowStore, action: RowAction): RowStore {
  switch (action.type) {
    case 'reset':
      return initGame([action.pegCount, action.colorCount])
    case 'submit':
      state.previousRows.push(state.currentRow)
      state.previousHints.push(state.currentHint)

      return {
        ...state,
        currentHint: [0, 0],
        currentRow: Array(state.pegCount).fill('')
      }
    case 'peg':
      let nextRow = state.currentRow
      nextRow[action.idx] = action.color

      return {
        ...state,
        currentRow: nextRow,
      }
    case 'hint':
      let nextHint = state.currentHint
      nextHint[action.idx] = action.count

      return {
        ...state,
        currentHint: nextHint,
      }
    case 'solution':
      return {
        ...state,
        finished: true,
        currentRow: action.colors,
        currentHint: [state.pegCount, 0]
      }
  }
}


type SolverStats = {
  pre_prune_guesses: number,
  post_prune_guesses: number,
  possibilities: number,
  time: number,
  worstCase: number
}

type SolverState = { type: 'solved', stats: SolverStats } | { type: 'solving' } | { type: 'idle' }


export const Game = (props: { solverConstructor: SolverType }) => {
  const totalRows = 9;
  const maxPegs = 7;
  const maxColors = 8;

  const [nextPegCount, setNextPegCount] = useState(5);
  const [nextColorCount, setNextColorCount] = useState(8);

  const [activeColor, setActiveColor] = useState('red');

  const [gameState, dispatch] = useReducer(gameReducer, [nextPegCount, nextColorCount], initGame);

  const [solverState, setSolverState] = useState<SolverState>({type: 'idle'})

  const solver = useRef<ConcreteSolver | null>(null);

  useEffect(() => {
    initSolver(nextPegCount, nextColorCount)
  }, [])

  async function initSolver(pegCount: number, colorCount: number) {
    solver.current = await new props.solverConstructor(pegCount, colorCount);
  }

  const onResetGame = () => {
    initSolver(nextPegCount, nextColorCount)
    dispatch({ type: 'reset', pegCount: nextPegCount, colorCount: nextColorCount })
  }

  const onSubmitGuess = async () => {
    const guessAsUints = Uint8Array.from(gameState.currentRow.map((color: string) => ColorNames.indexOf(color)))
    const remaining = await solver.current!.addGuess(guessAsUints, gameState.currentHint[0], gameState.currentHint[1])

    dispatch({ type: 'submit' })

    if (remaining == 1) {
      const finalAwnser = await solver.current?.get_final_awnser();
      const solution = Array.from(finalAwnser!).map((c) => ColorNames[c])
      dispatch({ type: 'solution', colors: solution })
    }

  }

  const onFindOptimalGuess = async () => {
    setSolverState({type: 'solving'});
    const { guess, ...stats } = await solver.current!.minMax();

    for (var [idx, c] of guess.entries()) {
      dispatch({ type: 'peg', idx: idx, color: ColorNames[c] })
    }

    setSolverState({type: 'solved', stats})
  }

  return (
    <div className='game-container'>
      <Menu
        canSubmit={!gameState.currentRow.includes('') && (gameState.currentHint[0] + gameState.currentHint[1] <= gameState.currentRow.length)}
        canReset={!!gameState.previousRows.length || gameState.colorCount != nextColorCount || gameState.pegCount != nextPegCount}
        onFindGuess={onFindOptimalGuess}
        onSubmit={onSubmitGuess}
        onReset={onResetGame}
        pegCount={nextPegCount}
        colorCount={nextColorCount}
        onColorCountChange={setNextColorCount}
        onPegCountChange={setNextPegCount}
        maxPegs={maxPegs}
        maxColors={maxColors}
      />

      <SolverInfo state={solverState} />

      <Colors
        colors={ColorNames.slice(0, gameState.colorCount)}
        activeColor={activeColor}
        onClick={setActiveColor} />

      <Board
        onPegChange={(idx: number) => dispatch({ type: 'peg', idx: idx, color: activeColor })}
        onHintChange={(idx: number, count: number) => dispatch({ type: 'hint', idx: idx, count: count })}
        {...gameState}
        currentHint={gameState.currentHint}
        currentRow={gameState.currentRow}
        activeRow={gameState.previousHints.length}
        pegCount={gameState.pegCount}
        totalRows={totalRows}
        finished={gameState.finished} />
    </div>
  );
}

const SolverInfo = ({state}: { state: SolverState }) => {
  return (
    <div className='state'>
      {state.type === 'solving' && <Grid key={state.type} ariaLabel="loading-indicator" />}
      {state.type === 'solved' && <Info {...state.stats} key={state.type} />}
    </div>
  )
  
}

const Info = (props: SolverStats) => {
  return (
    <div className='stats'>
      <span>{props.post_prune_guesses * props.possibilities} positions evaluated in {Math.round(props.time)}ms</span>
      <span>Reduced {props.pre_prune_guesses} to {props.post_prune_guesses} non-equivalent guesses</span>
      <span>{props.possibilities} possibilities remaining - At most {props.worstCase} after this guess</span>
    </div>
  )
}