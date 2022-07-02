import * as Comlink from 'comlink';
import init, { initThreadPool, Solver } from 'wasm-mastermind';

export type OptimalGuess = {
    guess: Uint8Array,
    pre_prune_guesses: number,
    post_prune_guesses: number,
    possibilities: number,
    time: number,
    worstCase: number
}

//We need to wrap the Solver again, because Comlink does not wrap the return type of functions
//minMax() returns an Guess object which uses local pointers and is worthless outside the worker context
export class WrappedSolver {
    solver: Solver
    constructor(pegs: number, colors: number) {
        this.solver = Solver.init(pegs, colors);
    }

    addGuess(guess: Uint8Array, blackPegs: number, whitePegs: number): number {
        return this.solver.add_guess_result(guess, blackPegs, whitePegs)
    }

    reset() {
        this.solver.reset_guesses()
    }

    minMax(): OptimalGuess {
        const start = performance.now();
        const minMax = this.solver.min_max()
        const end = performance.now();

        return {
            guess: minMax.get_guess(),
            pre_prune_guesses: minMax.get_pre_prune_guesses(),
            post_prune_guesses: minMax.get_post_prune_guesses(),
            possibilities: minMax.get_possibilities(),
            time: end - start,
            worstCase: minMax.get_worst_case()
        }
        
    }

    get_final_awnser(): Uint8Array | undefined {
        return this.solver.get_final_awnser()
    }
}



async function initWorkers() {
    console.log("Loading wasm")
    await init();

    console.log("Creating thread pool")
    await initThreadPool(navigator.hardwareConcurrency);

    console.log("Init complete")

    return Comlink.proxy(WrappedSolver)
}

Comlink.expose({ initSolver: initWorkers()});