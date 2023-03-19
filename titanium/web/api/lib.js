/**
 * # Titanium API
 * ##
 * @module api
*/

// deno-lint-ignore-file


(function (global, factory) {
	typeof exports === 'object' && typeof module !== 'undefined' ? factory(exports) :
	typeof define === 'function' && define.amd ? define(['exports'], factory) :
	(global = typeof globalThis !== 'undefined' ? globalThis : global || self, factory(global.TITANIUM = {}));
})(this, (function (exports) { exports.__private__ = {}; 'use strict';
	class TitaniumConnectError extends Error {
		constructor(c) {
			super("Failed to connect to " +
				c.connection.secure ? "wss" : "ws" +
				"" +
				c.connection.host +
				":" +
				c.connection.port +
			"!");
			this.name = "TitaniumConnectError";
		}
	}
	let __private__;(function(__private__){
		class JSLib {
			#code;
			constructor(code) {
				this.#code = code;
			}
			getCode() {
				return this.#code;
			}
			inject() {
				try {
					eval(this.#code);
				}
				catch (e) {
					console.error("Error injecting JSLIB");
					console.error(e);
				}
			}
		}__private__.JSLib = JSLib;
		class Option {
			#valid;
			#value;
			static of(value) {
				if (value === undefined)
					throw new TypeError("Value cannot be undefined");
				return new Option(value);
			}
			static ofNullable(value) {
				return new Option(value);
			}
			static empty() {
				return new Option(undefined);
			}
			constructor(value) {
				if (value === undefined) this.#valid = false;
				else this.#valid = true;
				this.#value = value;
			}
			isPresent() {
				return this.#valid && this.#value !== undefined;
			}
			getStrict() {
				if (this.#value === undefined) throw new TypeError("Option is empty");
				return this.#value;
			}
			get() {
				return this.#value;
			}
			ifPresent(consumer) {
				if (this.#value !== undefined) consumer(this.#value);
			}
			ifPresentOrElse(consumer, orElse) {
				if (this.#value !== undefined) consumer(this.#value);
				else orElse();
			}
			orElseGet(supplier) {
				if (this.#value !== undefined) return this.#value;
				else return supplier();
			}
			orElse(other) {
				if (this.#value !== undefined) return this.#value;
				else return other;
			}
			orElseThrow(error) {
				if (this.#value !== undefined) return this.#value;
				else throw error;
			}
			map(mapper) {
				if (this.#value !== undefined) return Option.ofNullable(mapper(this.#value));
				else return Option.empty();
			}
			flatMap(mapper) {
				if (this.#value !== undefined) return mapper(this.#value);
				else return Option.empty();
			}
			filter(predicate) {
				if (this.#value !== undefined) {
					if (predicate(this.#value)) return this;
					else return Option.empty();
				} else return Option.empty();
			}
			equals(other) {
				if (this.isPresent() && other.isPresent()) {
					return this.#value === other.#value;
				} else if (!this.isPresent() && !other.isPresent()) {
					return true;
				} else return false;
			}
			set(value) {
				this.#value = value;
				this.#valid = true;
			}
			clear() {
				this.#value = undefined;
				this.#valid = false;
			}
			setNullable(value) {
				this.#value = value;
				if (value === undefined) this.#valid = false;
				else this.#valid = true;
			}
		}__private__.Option = Option;
		class EventEmitter {
			_events_ = new Map();
			on(event, listener) {
				if (!this._events_.has(event)) this._events_.set(event, new Set());
				this._events_.get(event).add(listener);
				return this;
			}
			once(event, listener) {
				const l = listener;
				l.__once__ = true;
				return this.on(event, l);
			}
			off(event, listener) {
				if ((event === undefined || event === null) && listener) throw new Error("Why is there a listener defined here?");
				else if ((event === undefined || event === null) && !listener) this._events_.clear();
				else if (event && !listener) this._events_.delete(event);
				else if (event && listener && this._events_.has(event)) {
					const _ = this._events_.get(event);
					_.delete(listener);
					if (_.size === 0) this._events_.delete(event);
				} else;return this;
			}
			emitSync(event, ...args) {
				if (!this._events_.has(event)) return this;
				const _ = this._events_.get(event);
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
			async emit(event, ...args) {
				if (!this._events_.has(event)) return this;
				const _ = this._events_.get(event);
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
			queue(event, ...args) {
				(async () => await this.emit(event, ...args))().catch(console.error);
				return this;
			}
			pull(event, timeout) {
				return new Promise(async (resolve, reject) => {
					let timeoutId;
					const listener = (...args) => {
						if (timeoutId !== null)
							clearTimeout(timeoutId);
						resolve(args);
					};
					timeoutId = typeof timeout !== "number"
						? null
						: setTimeout(() => (
							this.off(event, listener),
							reject(
								new Error("Timed out!")
							)
						));
					this.once(event, listener);
				});
			}
		}__private__.EventEmitter = EventEmitter;
	})(__private__ || (__private__ = {}));
	function e2c(i) {
		if ("strictConnection" in i) return {
			host: i["strictConnection"].host,
			port: i["strictConnection"].port,
			secure: i["strictConnection"].secure,
		};
		return {
			host: i.connection.host,
			port: i.connection.port,
			secure: i.connection.secure,
		};
	}
	function c2e(i) {
		if ("host" in i && "port" in i && "secure" in i) {
			if (typeof i.host === "string" &&
				typeof i.port === "number" &&
				typeof i.secure === "boolean"
			) return {
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
			};
			return {
				connection: {
					host: i.host,
					port: i.port,
					secure: i.secure,
				}
			};
		} else throw new TypeError("Invalid connection");
	}
	function iConnectionParser(connection) {
		if (connection === undefined) return {
			host: "localhost",
			port: 8080,
			secure: false
		};
		if (typeof connection !== "string") return {
			host: connection.host ?? "localhost",
			port: connection.port ?? 8080,
			secure: connection.secure ?? false
		};
		const newHost = {
			host: "localhost",
			port: 8080,
			secure: false
		};
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
	class ConnctedTitaniumServer {
		#host;
		#port;
		#secure;
		#ws;
		#eventHandler;
		constructor(c, ws) {
			this.#host = c.strictConnection.host;
			this.#port = c.strictConnection.port;
			this.#secure = c.strictConnection.secure;
			this.#ws = ws;
			this.#eventHandler = new __private__.EventEmitter();
			this.#ws.onopen = () => {
				this.#eventHandler.emitSync("open");
			};
			this.#ws.onclose = () => {
				this.#eventHandler.emitSync("close");
			};
			this.#ws.onmessage = (msg) => {
				this.#eventHandler.emitSync("message", msg.data);
			};
		}
		get strictConnection() {
			return {
				host: this.#host,
				port: this.#port,
				secure: this.#secure
			};
		}
		get connection() {
			return this.strictConnection;
		}
		getHost() {
			return this.#host;
		}
		getPort() {
			return this.#port;
		}
		isSecure() {
			return this.#secure;
		}
		disconnect() {
			return new TitaniumServer(this);
		}
		on(event, listener) {
			this.#eventHandler.on(event, listener);
		}
		once(event, listener) {
			this.#eventHandler.once(event, listener);
		}
		off(event, listener) {
			this.#eventHandler.off(event, listener);
		}
		send(data) {
			this.#ws.send(data);
		}
	}
	class TitaniumServer {
		#host;
		#port;
		#secure;
		constructor(c) {
			this.#host = c.strictConnection.host;
			this.#port = c.strictConnection.port;
			this.#secure = c.strictConnection.secure;
		}
		get strictConnection() {
			return {
				host: this.#host,
				port: this.#port,
				secure: this.#secure
			};
		}
		get connection() {
			return this.strictConnection;
		}
		getHost() {
			return this.#host;
		}
		getPort() {
			return this.#port;
		}
		isSecure() {
			return this.#secure;
		}
		connect() {
			try {
				const ws = new WebSocket(`${this.#secure ? "wss" : "ws"}://${this.#host}:${this.#port}`);
				return new ConnctedTitaniumServer(this, ws);
			} catch (_) {
				throw new TitaniumConnectError(this);
			}
		}
	}
	let __main__ = __private__.Option.empty();
	function main(callback) {
		__main__ = __private__.Option.of(callback);
	}
	function __run_main__() {
		__main__.ifPresent((c) => c());
	}
	function loadJSLIB(uri, blocking) {
		blocking = blocking === undefined ? true : blocking;
		let __e__ = "";
		try {
			const __r__ = new XMLHttpRequest();
			__r__.onreadystatechange = function () {
				if (__r__.readyState == 4) {
					if (__r__.status == 200) {
						__e__ = __r__.responseText;
					}
					else {
						console.error("Error Loading Library '" +
							uri + "', Server returned status code " +
							__r__.status);
						__e__ = "";
					}
				}
			};
			__r__.open("GET", uri, !blocking);
			__r__.send();
		}
		catch (__f__) {
			console.error("Error loading " + uri);
			console.error(__f__);
		}
		finally {
			return new __private__.JSLib(__e__);
		}
	}
	function connect(host) {
		const server = new TitaniumServer(c2e(iConnectionParser(host)));
		return server.connect();
	}
	exports.__private__ = __private__;
	exports.loadJSLIB = loadJSLIB;
	exports.connect = connect;
	exports.main = main;
	exports.__run_main__ = __run_main__;
}));