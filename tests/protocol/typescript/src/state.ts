export class State {
    private _middleware: boolean = false;

    public setMiddleware(middleware: boolean) {
        this._middleware = middleware;
    }

    public getMiddleware(): boolean {
        return this._middleware;
    }

}

const state: State = new State();

export { state };