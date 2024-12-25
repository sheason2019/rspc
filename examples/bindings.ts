// My custom header
// This file was generated by [rspc](https://github.com/specta-rs/rspc). Do not edit this file manually.

export type ProceduresLegacy = { queries: { key: "cached"; input: any; result: any } | { key: "echo"; input: string; result: string } | { key: "error"; input: null; result: string } | { key: "login"; input: any; result: any } | { key: "me"; input: any; result: any } | { key: "nested.hello"; input: null; result: string } | { key: "newstuff"; input: any; result: any } | { key: "newstuff2"; input: any; result: any } | { key: "newstuffpanic"; input: any; result: any } | { key: "newstuffser"; input: any; result: any } | { key: "panic"; input: null; result: null } | { key: "sfmPost"; input: any; result: any } | { key: "sfmPostEdit"; input: any; result: any } | { key: "transformMe"; input: null; result: string } | { key: "validator"; input: any; result: any } | { key: "version"; input: null; result: string } | { key: "withoutBaseProcedure"; input: any; result: any }; mutations: { key: "sendMsg"; input: string; result: string }; subscriptions: { key: "pings"; input: null; result: string } }

export type Procedures = {
	cached: { kind: "query", input: any, output: any, error: any },
	echo: { kind: "query", input: string, output: string, error: unknown },
	error: { kind: "query", input: null, output: string, error: unknown },
	login: { kind: "query", input: any, output: any, error: any },
	me: { kind: "query", input: any, output: any, error: any },
	nested: {
	hello: { kind: "query", input: null, output: string, error: unknown },
},
	newstuff: { kind: "query", input: any, output: any, error: any },
	newstuff2: { kind: "query", input: any, output: any, error: any },
	newstuffpanic: { kind: "query", input: any, output: any, error: any },
	newstuffser: { kind: "query", input: any, output: any, error: any },
	panic: { kind: "query", input: null, output: null, error: unknown },
	pings: { kind: "subscription", input: null, output: string, error: unknown },
	sendMsg: { kind: "mutation", input: string, output: string, error: unknown },
	sfmPost: { kind: "query", input: any, output: any, error: any },
	sfmPostEdit: { kind: "query", input: any, output: any, error: any },
	transformMe: { kind: "query", input: null, output: string, error: unknown },
	validator: { kind: "query", input: any, output: any, error: any },
	version: { kind: "query", input: null, output: string, error: unknown },
	withoutBaseProcedure: { kind: "query", input: any, output: any, error: any },
}