const fs = require("fs");
const path = require("path");

module.exports = {
	findBundle: () => [],
	validate(stats, stderr, options) {
		const bundleFiles = [
			path.resolve(__dirname, "../../../js/config/parsing/import-meta-object/bundle.js"),
			path.resolve(
				__dirname,
				"../../../js/runtime-mode-config/parsing/import-meta-object/bundle.js"
			)
		].filter(file => fs.existsSync(file));

		expect(bundleFiles.length).toBeGreaterThan(0);
		for (const file of bundleFiles) {
			const source = fs.readFileSync(file, "utf-8");
			expect(source).toContain("import.meta.url");
			expect(source).toContain("const webpackVersion = 5");
			expect(source).toContain("import.meta.rspackHash");
			expect(source).toContain("import.meta.customRuntimeValue");
			expect(source).toContain("unsupported import.meta.env");
			expect(source).toContain("./a.js");
			expect(source).not.toContain("import.meta.webpackContext");
			expect(source).not.toContain("import.meta.webpack;");
		}
	}
};
