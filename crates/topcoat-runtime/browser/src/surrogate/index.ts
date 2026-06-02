import { F64 } from "./f64";
import { Str } from "./str";
// biome-ignore lint/suspicious/noShadowRestrictedNames: Surrogate type
import { String } from "./string";

export * from "./f64";
export * from "./ref";
export * from "./signal";
export * from "./str";

export const surrogate = {
	f64(v: number) {
		return new F64(v);
	},
	str(v: string) {
		return new Str(v);
	},
	String(v: string) {
		return new String(v);
	},
};
