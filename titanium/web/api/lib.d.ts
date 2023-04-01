/**
 * @author AtomicGamer9523
 * @license MIT
 * @version 1.0.0-alpha.3
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
    host?: string;
    /**
     * ## `port`
     * ### port to connect to, can be any number
     * @type {number | undefined}
     * @default 80
    */
    port?: number;
    /**
     * ## `secure`
     * ### if the connection should be secure or not
     * @type {boolean | undefined}
     * @default false
    */
    secure?: boolean;
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
};
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
export declare class JSLib implements IJSLib {
    #private;
    constructor(code: string);
    getCode(): string;
    inject(): void;
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
export declare class Option<T> implements IOption<T> {
    #private;
    /**
     * ## `of`
     * ### creates a new option with a value
     * @param {T} value value to create the option with
     * @returns {Option<T>} new option
     * @throws {TypeError} if the value is undefined
     * @see {@link IOption#ofNullable}
     * @see {@link IOption#empty}
    */
    static of<T>(value: T): Option<T>;
    /**
     * ## `ofNullable`
     * ### creates a new option with a value or undefined
     * @param {T | undefined} value value to create the option with
     * @returns {Option<T>} new option
     * @see {@link IOption#of}
     * @see {@link IOption#empty}
    */
    static ofNullable<T>(value: T | undefined): Option<T>;
    /**
     * ## `empty`
     * ### creates a new empty option
     * @returns {Option<T>} new option
     * @see {@link IOption#of}
     * @see {@link IOption#ofNullable}
    */
    static empty<T>(): Option<T>;
    constructor(value: T | undefined);
    isPresent(): boolean;
    getStrict(): T;
    get(): T | undefined;
    ifPresent(consumer: (value: T) => void): void;
    ifPresentOrElse(consumer: (value: T) => void, orElse: () => void): void;
    orElseGet(supplier: () => T): T;
    orElse(other: T): T;
    orElseThrow(error: Error): T;
    map<R>(mapper: (value: T) => R): Option<R>;
    flatMap<R>(mapper: (value: T) => Option<R>): Option<R>;
    filter(predicate: (value: T) => boolean): Option<T>;
    equals(other: Option<T>): boolean;
    set(value: T): void;
    clear(): void;
    setNullable(value: T | undefined): void;
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
export declare class EventEmitter<E extends EventsType = {}> implements IEventEmitter<E> {
    private _events_;
    on<K extends keyof E>(event: K, listener: E[K]): this;
    once<K extends keyof E>(event: K, listener: E[K]): this;
    off<K extends keyof E>(event: K, listener: E[K]): this;
    off<K extends keyof E>(event: K): this;
    off(): this;
    emitSync<K extends keyof E>(event: K, ...args: Parameters<E[K]>): this;
    emit<K extends keyof E>(event: K, ...args: Parameters<E[K]>): Promise<this>;
    queue<K extends keyof E>(event: K, ...args: Parameters<E[K]>): this;
    pull<K extends keyof E>(event: K, timeout?: number): Promise<Parameters<E[K]>>;
}
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
export declare function loadJSLIB(uri: string, blocking?: boolean): IJSLib;
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
export declare function main(callback: () => void): void;
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
export declare function connect(host?: string | IConnection): IConnectedTitaniumServer;
//# sourceMappingURL=lib.d.ts.map