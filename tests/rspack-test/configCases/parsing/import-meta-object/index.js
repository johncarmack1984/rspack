const preservedUrl = import.meta.url;
const webpackVersion = import.meta.webpack;
const knownEnv = import.meta.env;
const preservedRspackHash = import.meta.rspackHash;
const preservedCustomValue = import.meta.customRuntimeValue;
const context = import.meta.webpackContext("./dir", {
	recursive: false
});

export const values = [
	preservedUrl,
	webpackVersion,
	knownEnv,
	preservedRspackHash,
	preservedCustomValue,
	context.keys()
];
