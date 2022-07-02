import React from 'react'

type BoardProps = {
  onPegChange: (id: number) => void,
  onHintChange: (kind: number, count: number) => void,
  previousRows: string[][],
  previousHints: [number, number][],
  currentRow: string[]
  currentHint: [number, number]
  activeRow: number,
  totalRows: number,
  pegCount: number,
  finished: boolean
}

export const Board = (props: BoardProps) => {
  let rows = [];
  let { activeRow, pegCount, previousRows, previousHints, currentRow, currentHint, ...rest } = props;
  for (let i = 0; i < props.totalRows; i++) {
    let colors = Array(pegCount).fill('')
    let hint: [number, number] = [0, 0];

    if (i < previousRows.length) {
      colors = previousRows[i]
      hint = previousHints[i]
    } else if (i == activeRow) {
      colors = currentRow
      hint = currentHint
    }

    rows.push(
      <Row
        key={i}
        activeRow={i == activeRow && !props.finished}
        pegCount={pegCount}
        hint={hint}
        colors={colors}
        {...rest} />
    )
  }
  return (
    <div className='board'>
      {rows}
    </div>
  )
}


type RowProps = {
  activeRow?: boolean,
  hint: [number, number],
  colors: string[],
  pegCount: number
  onHintChange: (pegIndex: number, count: number) => void
  onPegChange: (id: number) => void
}

const Row = (props: RowProps) => {
  let active = props.activeRow ? 'active' : '';
  return (
    <div className={'row ' + active}>
      <Circles onClick={props.activeRow ? props.onPegChange : () => {}} {...props} />
      {props.activeRow ? <HintInput onChange={props.onHintChange} {...props} /> : <Hint {...props} />}
    </div>
  )
}

type CirclesProps = {
  colors: string[],
  onClick: (id: number) => void
}

const Circles = (props: CirclesProps) => {
  let Pegs = []
  for (let i = 0; i < props.colors.length; i++) {
    Pegs.push(
      <Peg
        key={i}
        onClick={props.onClick}
        id={i}
        color={props.colors[i]}
      />)
  }

  return <div className='circles'> {Pegs} </div>
}

type PegProps = {
  id: number,
  color: string,
  onClick: (id: number) => void
}

const Peg = (props: PegProps) => {
  return (
    <span
      className={'peg ' + props.color}
      onClick={() => props.onClick(props.id)} >
    </span>
  )
}

type HintProps = {
  hint: [number, number],
  pegCount: number
}

const Hint = (props: HintProps) => {
  let hintFields = []
  for (let i = 0; i < props.pegCount; i++) {
    const hintClass = i < props.hint[0] ? "exact" : (i < props.hint[0] + props.hint[1]) ? "partial" : ""

    hintFields.push(<span
      key={i}
      className={hintClass} />)
  }
  return (
    <div className='hints'>
      {hintFields}
    </div>
  )
}

type HintInputProps = {
  hint: [number, number]
  pegCount: number,
  onChange: (pegIndex: number, count: number) => void
}

const HintInput = (props: HintInputProps) => {
  return (
    <div className='hintsInput'>
      <div className='hintRow'><span>Black:</span><input type="number" value={props.hint[0]} onChange={(e) => props.onChange(0, parseInt(e.target.value))} min={0} max={props.pegCount} style={{ width: "2em", float: 'right' }} /></div>
      <div className='hintRow'><span>White:</span><input type="number" value={props.hint[1]} onChange={(e) => props.onChange(1, parseInt(e.target.value))} min={0} max={props.pegCount} style={{ width: "2em", float: 'right' }} /></div>
    </div>
  )
}