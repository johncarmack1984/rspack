it("should preserve whole-object import.meta.env definitions", () => {
	const env = import.meta.env;
	expect(env.MODE).toBe("production");
	expect(env.FEATURE).toBe("enabled");
	expect(env.UNKNOWN).toBe(undefined);
	expect(import.meta.env.UNKNOWN).toBe(undefined);
});
