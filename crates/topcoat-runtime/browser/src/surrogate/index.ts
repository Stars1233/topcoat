import { f64 } from "./f64";

export * from "./f64";
export * from "./ref";
export * from "./signal";

export const surrogate = {
	f64(v: number) {
		return new f64(v);
	},
};
