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

export type Result<E, T> = Success<E> | Error<T>;