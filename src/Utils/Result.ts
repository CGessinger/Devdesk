export interface Error<T> {
    kind: "error";
    value: T;
}

export interface Success<E> {
    kind: "success";
    value: E;
}

export function Err<T>(value: T): Error<T> {
    return { 
        value,
        kind: "error"
    };
}

export function Ok<E>(val: E): Success<E> {
    return { 
        value: val,
        kind: "success"
    };
}

export function isErr<E, T>(res: Result<E, T>): boolean {
    return res.kind === "error";
}

export function unwrap<E, T>(res: Result<E, T>): E {
    if (isErr(res)) {
        throw new Error(`unwrap called on error: ${res.value}`);
    }
    return res.value as E;
}

export type Result<E, T> = Success<E> | Error<T>;
export type PromisedResult<E, T> = Promise<Result<E, T>>;