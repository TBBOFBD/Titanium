/**
 * @author AtomicGamer9523
 * @license MIT
 * @version 1.0.0-alpha.4
 * @description Titanium Web API Library
*/

// deno-lint-ignore-file

/**
 * # `IConnection`
 * ## interface for a connection where all properties are optional
 * ### Example:
 * ```js
 * const connection: IConnection = {
 *    host: "localhost",
 *    port: 80,
 *    secure: false
 * };
 * ```
 * ### Or even easier:
 * ```js
 * const connection: IConnection = {
 *    host: "localhost"
 * };
 * ```
*/
interface IConnection {
    /**
     * ## `host`
     * ### host to connect to, can be an ip or a domain
     * @type {string | undefined}
     * @default "localhost"
    */
    host?: string,
    /**
     * ## `port`
     * ### port to connect to, can be any number
     * @type {number | undefined}
     * @default 80
    */
    port?: number,
    /**
     * ## `secure`
     * ### if the connection should be secure or not
     * @type {boolean | undefined}
     * @default false
    */
    secure?: boolean
}
/**
 * # `IConnectable`
 * ## interface for a connectable object
*/
interface IConnectable {
    get connection(): IConnection;
}
/**
 * # `IStrictConnection`
 * ## interface for a strict connection where all properties are required
*/
interface IStrictConnection extends IConnection {
    /**
     * ## `host`
     * ### host to connect to, can be an ip or a domain
     * @type {string}
     * @default "localhost"
    */
    host: string,
    /**
     * ## `port`
     * ### port to connect to, can be any number
     * @type {number}
     * @default 80
    */
    port: number,
    /**
     * ## `secure`
     * ### if the connection should be secure or not
     * @type {boolean}
     * @default false
    */
    secure: boolean
}
/**
 * # `IStrictConnectable`
 * ## interface for a strict connectable object
 * ### Example:
 * ```js
 * const connection: IStrictConnection = {
 *     host: "localhost",
 *     port: 80,
 *     secure: false
 * };
 * ```
*/
interface IStrictConnectable extends IConnectable {
    get strictConnection(): IStrictConnection;
}
/**
 * # `TitaniumConnectError`
 * ## error thrown when the connection fails
 * @param {IConnection} c connection that failed
 * @extends {Error}
 */
class TitaniumConnectError<C extends IStrictConnectable> extends Error {
    constructor(c: C) {
        super(
            "Failed to connect to " +
                c.connection.secure ? "wss" : "ws" +
                "" +
                c.connection.host +
                ":" +
                c.connection.port +
            "!"
        );
        this.name = "TitaniumConnectError";
    }
}
/**
 * # `ITitaniumServer`
 * ## interface for a non-connected server
*/
interface ITitaniumServer {
    /**
     * ## `getHost`
     * ### gets the host of the server
     * @returns {string}
    */
    getHost(): string;
    /**
     * ## `getPort`
     * ### gets the port of the server
     * @returns {number}
    */
    getPort(): number;
    /**
     * ## `isSecure`
     * ### checks if the server is secure
     * @returns {boolean}
     * @default false
     * @see {@link https://developer.mozilla.org/en-US/docs/Web/API/WebSocket#Properties}
    */
    isSecure(): boolean;
    /**
     * ## `connect`
     * ### connects to the server
     * @returns {IConnectedTitaniumServer}
     * @throws {TitaniumConnectError}
    */
    connect(): IConnectedTitaniumServer;
}
/**
 * # `IConnectedTitaniumServer`
 * ## interface for a connected server
 * ### Example:
 * ```js
 * const server = TITANIUM.connect();
 * server.on("message", (message) => {
 *    console.log(message);
 * });
 * server.send("Hello World!");
 * ```
*/
interface IConnectedTitaniumServer {
    /**
     * ## `getHost`
     * ### gets the host of the server
     * @returns {string}
    */
    getHost(): string;
    /**
     * ## `getPort`
     * ### gets the port of the server
     * @returns {number}
    */
    getPort(): number;
    /**
     * ## `isSecure`
     * ### checks if the server is secure
     * @returns {boolean}
     * @default false
     * @see {@link https://developer.mozilla.org/en-US/docs/Web/API/WebSocket#Properties}
    */
    isSecure(): boolean;
    /**
     * ## `disconnect`
     * ### disconnects from the server
     * @returns {ITitaniumServer}
    */
    disconnect(): ITitaniumServer;
}
/**
 * # `Callback`
 * ## callback type
 * @type {(...args: any[]) => any | Promise<any>}
*/
type Callback = (...args: any[]) => any | Promise<any>;
/**
 * # `Listener`
 * ## listener type
 * @type {Callback & { __once__?: true; }}
*/
type Listener = Callback & { __once__?: true; };
/**
 * # `EventName`
 * ## event name type
 * @type {string}
*/
type EventName = string;
/**
 * # `EventsType`
 * ## events type
 * @type {Record<string, Callback>}
*/
type EventsType = {
    [key: string]: Callback;
}
/**
 * # `IEventEmitter`
 * ## interface for an event emitter
 * ### Example:
 * ```js
 * const emitter = new EventEmitter();
 * emitter.on("message", (message) => {
 *    console.log(message);
 * });
 * emitter.emit("message", "Hello World!");
 * ```
 * @class
 * @param {E} [events={}] events to listen for
 * @template E events type
*/
export interface IEventEmitter<E extends EventsType = {}> {
    /**
     * ## `on`
     * ### adds an event listener
     * @param {string} event event to listen for
     * @param {Callback} listener listener to add
     * @returns {EventEmitter}
    */
    on<K extends keyof E>(event: K, listener: E[K]): this;
    /**
     * ## `on`
     * ### adds an event listener
     * @param {string} event event to listen for
     * @param {Callback} listener listener to add
     * @returns {EventEmitter}
    */
    on(event: EventName, listener: Callback): this;
    /**
     * ## `once`
     * ### adds an event listener that only fires once
     * @param {string} event event to listen for
     * @param {Callback} listener listener to add
     * @returns {EventEmitter}
    */
    once<K extends keyof E>(event: K, listener: E[K]): this;
    /**
     * ## `once`
     * ### adds an event listener that only fires once
     * @param {string} event event to listen for
     * @param {Callback} listener listener to add
     * @returns {EventEmitter}
    */
    once(event: EventName, listener: Callback): this;
    /**
     * ## `off`
     * ### removes an event listener
     * @param {string} event event to remove listener from
     * @param {Callback} listener listener to remove
     * @returns {EventEmitter}
    */
    off<K extends keyof E>(event: K, listener: E[K]): this;
    /**
     * ## `off`
     * ### removes all event listener from an event
     * @param {string} event event to remove listener from
    */
    off<K extends keyof E>(event: K): this;
    /**
     * ## `off`
     * ### removes all event listeners
     * @returns {EventEmitter}
    */
    off(): this;
    /**
     * ## `off`
     * ### removes an event listener
     * @param {string | undefined} event event to remove listener from
     * @param {Callback | undefined} listener listener to remove
     * @returns {EventEmitter}
     * @throws {Error} if there is a listener defined but no event
     * @throws {Error} if there is an event defined but no listener
     * @throws {Error} if there is an event and listener defined but the event does not exist
    */
    off(event?: EventName, listener?: Callback): this;
    /**
     * ## `emitSync`
     * ### emit an event waiting for each listener to return
     * @param {string} event event to emit
     * @param {...any[]} args arguments to pass to the listeners
     * @returns {EventEmitter}
     * @throws {Error} if the event does not exist
    */
    emitSync<K extends keyof E>(event: K, ...args: Parameters<E[K]>): this;
    /**
     * ## `emitSync`
     * ### emit an event waiting for each listener to return
     * @param {string} event event to emit
     * @param {...any[]} args arguments to pass to the listeners
     * @returns {EventEmitter}
     * @throws {Error} if the event does not exist
    */
    emitSync(event: EventName, ...args: Parameters<Callback>): this;
    /**
     * ## `emit`
     * ### emit an event without waiting for each listener to return
     * @param {string} event event to emit
     * @param {...any[]} args arguments to pass to the listeners
     * @returns {Promise<this>}
     * @throws {Error} if the event does not exist
     * @async
    */
    emit<K extends keyof E>(event: K, ...args: Parameters<E[K]>): Promise<this>;
    /**
     * ## `emit`
     * ### emit an event without waiting for each listener to return
     * @param {string} event event to emit
     * @param {...any[]} args arguments to pass to the listeners
     * @returns {Promise<this>}
     * @throws {Error} if the event does not exist
     * @async
    */
    emit(event: EventName, ...args: Parameters<Callback>): Promise<this>;
    /**
     * ## `queue`
     * ### queues an event to be emitted later after the current event was handled by all listeners
     * @param {string} event event to emit
     * @param {...any[]} args arguments to pass to the listeners
     * @returns {this}
     * @throws {Error} if the event does not exist
    */
    queue<K extends keyof E>(event: K, ...args: Parameters<E[K]>): this;
    /**
     * ## `queue`
     * ### queues an event to be emitted later after the current event was handled by all listeners
     * @param {string} event event to emit
     * @param {...any[]} args arguments to pass to the listeners
     * @returns {this}
     * @throws {Error} if the event does not exist
    */
    queue(event: EventName, ...args: Parameters<Callback>): this;
    /**
     * ## `pull`
     * ### pulls an event from the queue
     * @param {string} event event to pull
     * @param {number} [timeout] timeout in milliseconds
     * @returns {Promise<Parameters<Callback>>}
     * @throws {Error} if the event does not exist
    */
    pull<K extends keyof E>(event: K, timeout?: number): Promise<Parameters<E[K]>>;
    /**
     * ## `pull`
     * ### pulls an event from the queue
     * @param {string} event event to pull
     * @param {number} [timeout] timeout in milliseconds
     * @returns {Promise<Parameters<Callback>>}
     * @throws {Error} if the event does not exist
    */
    pull(event: EventName, timeout?: number): Promise<Parameters<Callback>>;
}
/**
 * ## `IOption`
 * ### Represents an optional value
 * @template T
 * @interface IOption
 * @public
 * @memberof TITANIUM
 * @description Represents an optional value
 * @example
 * ```js
 * const myOption = TITANIUM.Option.of("Hello World");
 * myOption.ifPresent((value) => {
 *    console.log(value);
 * });
 * ```
*/
export interface IOption<T> {
    /**
     * ## `isPresent`
     * ### Checks if the option is present
     * @returns {boolean}
    */
    isPresent(): boolean;
    /**
     * ## `getStrict`
     * ### Gets the value of the option, throws an error if the option is empty
     * @returns {T}
     * @throws {TypeError}
    */
    getStrict(): T;
    /**
     * ## `get`
     * ### Gets the value of the option, returns undefined if the option is empty
     * @returns {T | undefined}
    */
    get(): T | undefined;
    /**
     * ## `ifPresent`
     * ### Executes a function if the option is present
     * @param {(value: T) => void} consumer function to execute
     * @returns {void}
    */
    ifPresent(consumer: (value: T) => void): void;
    /**
     * ## `ifPresentOrElse`
     * ### Executes a function if the option is present, otherwise executes another function
     * @param {(value: T) => void} consumer function to execute if the option is present
     * @param {() => void} orElse function to execute if the option is empty
     * @returns {void}
    */
    ifPresentOrElse(consumer: (value: T) => void, orElse: () => void): void;
    /**
     * ## `orElseGet`
     * ### Gets the value of the option, otherwise executes a function and returns the result
     * @param {() => T} supplier function to execute if the option is empty
     * @returns {T}
    */
    orElseGet(supplier: () => T): T;
    /**
     * ## `orElse`
     * ### Gets the value of the option, otherwise returns a default value
     * @param {T} other default value to return if the option is empty
     * @returns {T}
    */
    orElse(other: T): T;
    /**
     * ## `orElseThrow`
     * ### Gets the value of the option, otherwise throws an error
     * @param {Error} error error to throw if the option is empty
     * @returns {T}
     * @throws {Error}
    */
    orElseThrow(error: Error): T;
    /**
     * ## `map`
     * ### Maps the value of the option to another value
     * @param {(value: T) => R} mapper function to map the value
     * @returns {IOption<R>}
     * @template R
    */
    map<R>(mapper: (value: T) => R): IOption<R>;
    /**
     * ## `flatMap`
     * ### Maps the value of the option to another option
     * @param {(value: T) => IOption<R>} mapper function to map the value
     * @returns {IOption<R>}
     * @template R
    */
    flatMap<R>(mapper: (value: T) => IOption<R>): IOption<R>;
    /**
     * ## `filter`
     * ### Filters the value of the option
     * @param {(value: T) => boolean} predicate function to filter the value
     * @returns {IOption<T>}
     * @template R
    */
    filter(predicate: (value: T) => boolean): IOption<T>;
    /**
     * ## `equals`
     * ### Checks if the option is equal to another option
     * @param {IOption<T>} other option to compare to
     * @returns {boolean}
     * @template R
    */
    equals(other: IOption<T>): boolean;
    /**
     * ## `set`
     * ### Sets the value of the option
     * @param {T} value value to set
     * @returns {void}
    */
    set(value: T): void;
    /**
     * ## `clear`
     * ### Clears the option
     * @returns {void}
    */
    clear(): void;
    /**
     * ## `setNullable`
     * ### Sets the value of the option, if the value is undefined, the option will be cleared
     * @param {T | undefined} value value to set
     * @returns {void}
    */
    setNullable(value: T | undefined): void;
}
/**
 * ## `IJSLib`
 * ### Represents a JSLIB
 * @interface IJSLib
 * @public
 * @memberof TITANIUM
 * @description Represents a JSLIB
 * @example
 * ```js
 * const myLib = TITANIUM.loadJSLIB("myLib.js");
 * myLib.inject();
 * ```
 * @example
 * ```js
 * TITANIUM.loadJSLIB("myLib.js").inject();
 * ```
*/
export interface IJSLib {
    /**
     * # `getCode`
     * ## Gets the code of the JSLIB
     * @returns {string}
     * @memberof JSLib
     * @instance
     * @public
     * @function
     * @name getCode
     * @type {Function}
    */
    getCode(): string;
    /**
     * # `inject`
     * ## Injects the JSLIB into the current scope
     * @returns {void}
     * @memberof JSLib
     * @instance
     * @public
     * @function
     * @name inject
     * @type {Function}
     * @description Injects the JSLIB into the current scope
    */
    inject(): void;
}
/**
 * ## `JSLib`
 * ### Represents a JSLIB
 * @class JSLib
 * @public
 * @memberof TITANIUM
 * @description Represents a JSLIB
*/
export class JSLib
    implements IJSLib
{
    #code: string;
    constructor(code: string) {
        this.#code = code;
    }
    public getCode(): string {
        return this.#code;
    }
    public inject(): void {
        try {
            eval(this.#code);
        } catch (e) {
            console.error("Error injecting JSLIB");
            console.error(e);
        }
    }
}
/**
 * # `Option`
 * ## class for optional values
 * ### Example:
 * ```js
 * const option: Option<string> = Option.of("Hello World");
 * ```
 * @class Option
 * @public
 * @memberof TITANIUM
 * @description class for optional values
*/
export class Option<
    T
>
    implements IOption<T>
{
    #valid: boolean;
    #value: T | undefined;
    /**
     * ## `of`
     * ### creates a new option with a value
     * @param {T} value value to create the option with
     * @returns {Option<T>} new option
     * @throws {TypeError} if the value is undefined
     * @see {@link IOption#ofNullable}
     * @see {@link IOption#empty}
    */
    public static of<T>(value: T): Option<T> {
        if (value === undefined) throw new TypeError("Value cannot be undefined");
        return new Option(value);
    }
    /**
     * ## `ofNullable`
     * ### creates a new option with a value or undefined
     * @param {T | undefined} value value to create the option with
     * @returns {Option<T>} new option
     * @see {@link IOption#of}
     * @see {@link IOption#empty}
    */
    public static ofNullable<T>(value: T | undefined): Option<T> {
        return new Option(value);
    }
    /**
     * ## `empty`
     * ### creates a new empty option
     * @returns {Option<T>} new option
     * @see {@link IOption#of}
     * @see {@link IOption#ofNullable}
    */
    public static empty<T>(): Option<T> {
        return new Option<T>(undefined);
    }
    constructor(value: T | undefined) {
        if (value === undefined) this.#valid = false;
        else this.#valid = true;
        this.#value = value;
    }
    public isPresent(): boolean {
        return this.#valid && this.#value !== undefined;
    }
    public getStrict(): T {
        if (this.#value === undefined) throw new TypeError("Option is empty");
        return this.#value;
    }
    public get(): T | undefined {
        return this.#value;
    }
    public ifPresent(consumer: (value: T) => void): void {
        if (this.#value !== undefined) consumer(this.#value);
    }
    public ifPresentOrElse(consumer: (value: T) => void, orElse: () => void): void {
        if (this.#value !== undefined) consumer(this.#value);
        else orElse();
    }
    public orElseGet(supplier: () => T): T {
        if (this.#value !== undefined) return this.#value;
        else return supplier();
    }
    public orElse(other: T): T {
        if (this.#value !== undefined) return this.#value;
        else return other;
    }
    public orElseThrow(error: Error): T {
        if (this.#value !== undefined) return this.#value;
        else throw error;
    }
    public map<R>(mapper: (value: T) => R): Option<R> {
        if (this.#value !== undefined) return Option.ofNullable(mapper(this.#value));
        else return Option.empty();
    }
    public flatMap<R>(mapper: (value: T) => Option<R>): Option<R> {
        if (this.#value !== undefined) return mapper(this.#value);
        else return Option.empty();
    }
    public filter(predicate: (value: T) => boolean): Option<T> {
        if (this.#value !== undefined) {
            if (predicate(this.#value)) return this;
            else return Option.empty();
        } else return Option.empty();
    }
    public equals(other: Option<T>): boolean {
        if (this.isPresent() && other.isPresent()) {
            return this.#value === other.#value;
        } else if (!this.isPresent() && !other.isPresent()) {
            return true;
        } else return false;
    }
    public set(value: T): void {
        this.#value = value;
        this.#valid = true;
    }
    public clear(): void {
        this.#value = undefined;
        this.#valid = false;
    }
    public setNullable(value: T | undefined): void {
        this.#value = value;
        if (value === undefined) this.#valid = false;
        else this.#valid = true;
    }
}
/**
 * # `EventEmitter`
 * ## class for event emitters
 * ### Example:
 * ```js
 * const emitter = new EventEmitter();
 * emitter.on("event", (data) => {
 *    console.log(data);
 * });
 * emitter.emit("event", "Hello World");
 * ```
 * @class EventEmitter
 * @public
 * @memberof TITANIUM
 * @description class for event emitters
*/
export class EventEmitter<
    E extends EventsType = {}
>
    implements IEventEmitter<E>
{
    private _events_: Map<keyof E, Set<Listener>> = new Map();
    public on<K extends keyof E>(event: K, listener: E[K]): this;
    public on(event: EventName, listener: Callback): this {
        if (!this._events_.has(event)) this._events_.set(event, new Set());
        this._events_.get(event)!.add(listener);
        return this;
    }
    public once<K extends keyof E>(event: K, listener: E[K]): this;
    public once(event: EventName, listener: Callback): this {
        const l: Listener = listener;
        l.__once__ = true;
        return this.on(event, l as any);
    }
    public off<K extends keyof E>(event: K, listener: E[K]): this;
    public off<K extends keyof E>(event: K): this;
    public off(): this;
    public off(event?: EventName, listener?: Callback): this {
        if ((event === undefined || event === null) && listener)
            throw new Error("Why is there a listener defined here?");
        else if ((event === undefined || event === null) && !listener)
            this._events_.clear();
        else if (event && !listener)
            this._events_.delete(event);
        else if (event && listener && this._events_.has(event)) {
            const _ = this._events_.get(event)!;
            _.delete(listener);
            if (_.size === 0) this._events_.delete(event);
        } else;
        return this;
    }
    public emitSync<K extends keyof E>(event: K, ...args: Parameters<E[K]>): this;
    public emitSync(event: EventName, ...args: Parameters<Callback>): this {
        if (!this._events_.has(event)) return this;
        const _ = this._events_.get(event)!;
        for (const [, listener] of _.entries()) {
            const r = listener(...args);
            if (r instanceof Promise) r.catch(console.error);
            if (listener.__once__) {
                delete listener.__once__;
                _.delete(listener);
            }
        }
        if (_.size === 0) this._events_.delete(event);
        return this;
    }
    public async emit<K extends keyof E>(event: K, ...args: Parameters<E[K]>): Promise<this>;
    public async emit(event: EventName, ...args: Parameters<Callback>): Promise<this> {
        if (!this._events_.has(event)) return this;
        const _ = this._events_.get(event)!;
        for (const [, listener] of _.entries()) {
            try {
                await listener(...args);
                if (listener.__once__) {
                    delete listener.__once__;
                    _.delete(listener);
                }
            } catch (error) {
                console.error(error);
            }
        }
        if (_.size === 0) this._events_.delete(event);
        return this;
    }
    public queue<K extends keyof E>(event: K, ...args: Parameters<E[K]>): this;
    public queue(event: EventName, ...args: Parameters<Callback>): this {
        (async () => await this.emit(event, ...args as any))().catch(console.error);
        return this;
    }
    public pull<K extends keyof E>(event: K, timeout?: number): Promise<Parameters<E[K]>>;
    public pull(event: EventName, timeout?: number): Promise<Parameters<Callback>> {
        return new Promise(async (resolve, reject) => {
            let timeoutId: number | null
            const listener = (...args: any[]) => {
                if (timeoutId !== null) clearTimeout(timeoutId);
                resolve(args);
            };

            timeoutId = typeof timeout !== "number"
                ? null
                : setTimeout(() => (
                    this.off(event, listener as any),
                    reject(
                        new Error("Timed out!")
                    )
                ))
                ;
            this.once(event, listener as any);
        });
    }
}
/**
 * ## `e2e`
 * ### casts a connectable to a connection
 * @param {C1 | C2} i
 * @returns {C3 | C4}
 * @throws {TypeError}
 * @template C1 extends IConnection
 * @template C2 extends IStrictConnection
 * @template C3 extends IConnectable
 * @template C4 extends IStrictConnectable
*/
function e2c<
    C1 extends IConnection,
    C2 extends IStrictConnection,
    C3 extends IConnectable,
    C4 extends IStrictConnectable,
>(
    i: C3 | C4
): C1 | C2 {
    if ("strictConnection" in i) {
        return {
            host: i["strictConnection"].host,
            port: i["strictConnection"].port,
            secure: i["strictConnection"].secure,
        } as C2;
    } else {
        return {
            host: i.connection.host,
            port: i.connection.port,
            secure: i.connection.secure,
        } as C1;
    }
}
/**
 * ## `c2e`
 * ### casts a connection to a connectable
 * @param {C1 | C2} i
 * @returns {C3 | C4}
 * @throws {TypeError}
 * @template C1 extends IConnection
 * @template C2 extends IStrictConnection
 * @template C3 extends IConnectable
 * @template C4 extends IStrictConnectable
*/
function c2e<
    C1 extends IConnection,
    C2 extends IStrictConnection,
    C3 extends IConnectable,
    C4 extends IStrictConnectable,
>(
    i: C1 | C2
): C3 | C4 {
    if ("host" in i && "port" in i && "secure" in i) {
        if (
            typeof i.host === "string" &&
            typeof i.port === "number" &&
            typeof i.secure === "boolean"
        ) {
            return {
                connection: {
                    host: i.host,
                    port: i.port,
                    secure: i.secure,
                },
                strictConnection: {
                    host: i.host,
                    port: i.port,
                    secure: i.secure,
                }
            } as C4;
        } else return {
            connection: {
                host: i.host,
                port: i.port,
                secure: i.secure,
            }
        } as C3;
    } else throw new TypeError("Invalid connection");
}
/**
 * ## `iConnectionParser`
 * ### parses a connection string or object into a connection object
 * @param {string | IConnection} connection
 * @returns {IStrictConnection}
*/
function iConnectionParser(connection?: string | IConnection): IStrictConnection {
    if (connection === undefined) return {
        host: "localhost",
        port: 8080,
        secure: false
    }
    if (typeof connection !== "string") return {
        host: connection.host ?? "localhost",
        port: connection.port ?? 8080,
        secure: connection.secure ?? false
    }
    const newHost: IStrictConnection = {
        host: "localhost",
        port: 8080,
        secure: false
    }
    if (connection.includes("://")) {
        const split = connection.split("://");
        newHost.secure = split[0] === "wss";
        const sp = split[1];
        if (sp.includes(":")) {
            const split = sp.split(":");
            newHost.host = split[0];
            newHost.port = parseInt(split[1]);
        } else {
            newHost.host = sp;
        }
    } else {
        if (connection.includes(":")) {
            const split = connection.split(":");
            newHost.host = split[0];
            newHost.port = parseInt(split[1]);
        } else {
            newHost.host = connection;
        }
    }
    return newHost;
}
/**
 * ## `ConnectedTitaniumServer`
 * ### a connected titanium server
 * @implements {IConnectedTitaniumServer}
 * @implements {IStrictConnectable}
 * @class
*/
class ConnctedTitaniumServer
    implements IConnectedTitaniumServer, IStrictConnectable
{
    #host: string;
    #port: number;
    #secure: boolean;
    #ws: WebSocket;
    #eventHandler: EventEmitter<{
        open(): void;
        close(): void;
        message(data: string): void;
        error(err: Error): void;
    }>;
    /**
     * ## `constructor`
     * ### creates a new connected titanium server
     * @param {IStrictConnectable} c strict connectable object
     * @param {WebSocket} ws websocket connection
     * @constructor
    */
    constructor(c: IStrictConnectable, ws: WebSocket) {
        this.#host = c.strictConnection.host;
        this.#port = c.strictConnection.port;
        this.#secure = c.strictConnection.secure;
        this.#ws = ws;
        this.#eventHandler = new EventEmitter<{
            open(): void;
            close(): void;
            message(data: string): void;
        }>();
        this.#ws.onopen = () => {
            this.#eventHandler.emitSync("open");
        };
        this.#ws.onclose = () => {
            this.#eventHandler.emitSync("close");
        };
        this.#ws.onmessage = (msg) => {
            this.#eventHandler.emitSync("message", msg.data as string);
        };
    }
    get strictConnection(): IStrictConnection {
        return {
            host: this.#host,
            port: this.#port,
            secure: this.#secure
        }
    }
    get connection(): IConnection {
        return this.strictConnection;
    }
    public getHost(): string {
        return this.#host;
    }
    public getPort(): number {
        return this.#port;
    }
    public isSecure(): boolean {
        return this.#secure;
    }
    public disconnect(): ITitaniumServer {
        return new TitaniumServer(this);
    }
    public on(
        event: "open" | "close" | "message",
        listener: (() => void) | (() => void) | ((data: string) => void)
    ): void {
        this.#eventHandler.on(event, listener);
    }
    public once(
        event: "open" | "close" | "message",
        listener: (() => void) | (() => void) | ((data: string) => void)
    ): void {
        this.#eventHandler.once(event, listener);
    }
    public off(
        event: "open" | "close" | "message",
        listener: (() => void) | (() => void) | ((data: string) => void)
    ): void {
        this.#eventHandler.off(event, listener);
    }
    public send(data: string): void {
        this.#ws.send(data);
    }
}
class TitaniumServer
    implements ITitaniumServer,
    IStrictConnectable {
    #host: string;
    #port: number;
    #secure: boolean;
    constructor(c: IStrictConnectable) {
        this.#host = c.strictConnection.host;
        this.#port = c.strictConnection.port;
        this.#secure = c.strictConnection.secure;
    }
    get strictConnection(): IStrictConnection {
        return {
            host: this.#host,
            port: this.#port,
            secure: this.#secure
        }
    }
    get connection(): IConnection {
        return this.strictConnection;
    }
    public getHost(): string {
        return this.#host;
    }
    public getPort(): number {
        return this.#port;
    }
    public isSecure(): boolean {
        return this.#secure;
    }
    public connect(): IConnectedTitaniumServer {
        try {
            const ws = new WebSocket(
                `${this.#secure ? "wss" : "ws"}://${this.#host}:${this.#port}`
            );
            return new ConnctedTitaniumServer(this, ws);
        } catch (_) {
            throw new TitaniumConnectError(this);
        }
    }
}

/**
 * Main function of the library
 * @type {Option<() => void>} an optional function that will be called when the library is loaded
 * @memberof TITANIUM
*/
let __main__: Option<() => void> = Option.empty();

/**
 * # `loadJSLIB`
 * ## Loads a JSLIB from an uri
 * ### Example:
 * ```js
 * const myLib = TITANIUM.loadJSLIB("myLib.js");
 * myLib.inject();
 * ```
 * ### Or even easier:
 * ```js
 * TITANIUM.loadJSLIB("myLib.js").inject();
 * ```
 * ### Async ?
 * #### Theoritcally, yes, but it's not recommended
 * @param {string} uri uri to load from, can be relative or absolute
 * @param {boolean | undefined} blocking if the code should wait for the file to load before continuing, defaults to 'true'
 * @returns {IJSLib} a JSLIB object that can be used to inject the code into the current scope
 * @function loadJSLIB
 * @memberof TITANIUM
 * @instance
 * @public
 * @type {Function}
 * @description Loads a JSLIB from an uri
 * @example
 * ```js
 * const myLib = TITANIUM.loadJSLIB("myLib.js");
 * myLib.inject();
 * ```
 * @example
 * ```js
 * TITANIUM.loadJSLIB("myLib.js").inject();
 * ```
*/
export function loadJSLIB(uri: string, blocking?: boolean): IJSLib {
    blocking = blocking === undefined ? true : blocking;
    let __e__ = "";
    try {
        const __r__ = new XMLHttpRequest();
        __r__.onreadystatechange = function () {
            if (__r__.readyState == 4) {
                if (__r__.status == 200) {
                    __e__ = __r__.responseText
                } else {
                    console.error(
                        "Error Loading Library '" +
                        uri + "', Server returned status code " +
                        __r__.status
                    );
                    __e__ = "";
                }
            }
        };
        __r__.open("GET", uri, !blocking);
        __r__.send();
    } catch (__f__) {
        console.error("Error loading " + uri);
        console.error(__f__);
    } finally {
        return new JSLib(__e__);
    }
}

/**
 * # `main`
 * ## sets the main function (entry point)
 * ### Example:
 * ```js
 * TITANIUM.main(() => {
 *    console.log("Hello World!");
 * });
 * ```
 * @param {() => void} callback the main function
 * @function main
 * @memberof TITANIUM
 * @instance
 * @public
 * @type {Function}
 * @description sets the main function (entry point)
*/
export function main(callback: () => void): void {
    __main__ = Option.of(callback);
}

/**
 * # `__run_main__`
 * ## runs the main function (entry point)
 * @function __run_main__
 * @memberof TITANIUM
 * @instance
 * @private
 * @type {Function}
 * @description runs the main function (entry point)
 * @NODECLARATIONEMIT
*/
export function __run_main__(): void {
    __main__.ifPresent((c) => c());
}

/**
 * # `connect`
 * ## Connects to a Titanium server
 * ### Example:
 * ```js
 * TITANIUM.connect();
 * ```
 * ### Or with a custom host:
 * ```js
 * TITANIUM.connect("myhost.com");
 * ```
 * @param {string | IConnection | undefined} host host to connect to, defaults to 'localhost'
 * @returns {IConnectedTitaniumServer} a new instance of the Titanium Server API
 * @function connect
 * @memberof TITANIUM
 * @instance
 * @public
 * @type {Function}
 * @description Connects to the local Titanium server
 * @example
 * ```js
 * TITANIUM.connect();
 * ```
*/
export function connect(
    host?: string | IConnection
): IConnectedTitaniumServer {
    const server = new TitaniumServer(
        c2e(
            iConnectionParser(host)
        )
    );
    return server.connect();
}