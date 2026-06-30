it("should expose NODE_ENV from mode (WebpackOptionsApply)", () => {
	const env = import.meta.env;
	expect(env.NODE_ENV).toBe("production");
});

it("should expose variables from EnvironmentPlugin", () => {
	const env = import.meta.env;
	expect(env.ENV_VAR_FROM_ENV).toBe("from_environment_plugin");
});

it("should expose variables from DefinePlugin", () => {
	const env = import.meta.env;
	expect(env.CUSTOM_VAR).toBe("custom_value");
});

it("should keep direct access and object replacement consistent for duplicate definitions", () => {
	const env = import.meta.env;
	expect(import.meta.env.ORDERED_VAR).toBe("first_define_plugin");
	expect(env.ORDERED_VAR).toBe(import.meta.env.ORDERED_VAR);
});

it("should not mirror import.meta.env definitions to process.env", () => {
	const env = import.meta.env;
	expect(env.ONLY_IMPORT_META).toBe("only_import_meta");
	expect(process.env.ONLY_IMPORT_META).not.toBe("only_import_meta");
});

it("should not collect user process.env definitions into import.meta.env", () => {
	const env = import.meta.env;
	expect(process.env.PROCESS_ONLY).toBe("process_only");
	expect(env.PROCESS_ONLY).toBe(undefined);
});

it("should emit __proto__ env keys as own properties", () => {
	const env = import.meta.env;
	expect(Object.prototype.hasOwnProperty.call(env, "__proto__")).toBe(true);
	expect(env.__proto__).toBe("proto_value");
});

it("should support typeof import.meta.env", () => {
	expect(typeof import.meta.env).toBe("object");
});

it("should evaluate typeof import.meta.env as 'object'", () => {
	const typeofEnv = typeof import.meta.env;
	expect(typeofEnv).toBe("object");
});

it("should treat import.meta.env as truthy", () => {
	if (import.meta.env) {
		expect(true).toBe(true);
	} else {
		throw new Error("import.meta.env should be truthy");
	}
});

it("should wrap import.meta.env object literals in expression context", () => {
	import.meta.env;
	expect(import.meta.env && true).toBe(true);
});

it("should treat import.meta.env.NOT_EXIST as falsy", () => {
	if (import.meta.env.NOT_EXIST) {
		throw new Error("import.meta.env should be falsy");
	} else {
		expect(true).toBe(true);
	}
});

it("should treat import.meta.env.NOT_EXIST as falsy", () => {
	const NOT_EXIST = import.meta.env.NOT_EXIST;
	if (NOT_EXIST) {
		throw new Error("import.meta.env should be falsy");
	} else {
		expect(true).toBe(true);
	}
});
