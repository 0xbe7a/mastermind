import React, { useEffect, useRef, useState } from 'react';
import { createRoot } from 'react-dom/client';
import { Game } from './game'
import './style.css';
import * as Comlink from 'comlink';
import type { WrappedSolver } from './wasm-workers';

export type ConcreteSolver = Comlink.Remote<WrappedSolver>
export type SolverType = Comlink.Remote<typeof WrappedSolver>

const Mastermind = () => {
  const [wasmLoaded, setWasmLoaded] = useState(false);
  const solver = useRef<SolverType | null>(null);
  
  useEffect(() => {
    const worker = new Worker(new URL('./wasm-workers', import.meta.url));
    Comlink.wrap<{initSolver: Promise<SolverType>}>(worker).initSolver.then((s) => {
      solver.current = s
      setWasmLoaded(true)
    })
  }, [])

  return (
    <div>
      <h2 className='title'> MasterMind Solver</h2>
      {wasmLoaded ? <Game solverConstructor={solver.current!}/> : <span>Loading wasm</span>}
    </div>
    );
}

const container = document.getElementById('root');
const root = createRoot(container!);
root.render(<React.StrictMode><Mastermind /></React.StrictMode>)
