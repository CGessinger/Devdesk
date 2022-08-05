interface ResultType<T> {
    kind: "success" | "error";
    value: T;
    is_ok(): boolean;
    is_err(): boolean;
    unwrap(): T;
}

interface Error<T> extends ResultType<T>{
    kind: "error";
    asRejected(): Promise<any>;
}

export const Err = <T>(value: T): Error<T> => {
    return {
        kind: "error",
        value,
        asRejected: () => Promise.reject(value),
        is_ok: () => false,
        is_err: () => true,
        unwrap: () => {
            throw new Error("Unwrap called on Error: ", value);
        }
    };
};

export interface Success<E> extends ResultType<E> {
    kind: "success";
    asResolved(): Promise<E>;
}

export function Ok<E>(value: E): Success<E> {
    return { 
        kind: "success",
        value,
        asResolved: () => Promise.resolve(value),
        is_ok: () => true,
        is_err: () => false,
        unwrap:() => value
    };
}

export type Result<E, T> = Success<E> | Error<T>;