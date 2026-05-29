import type { WriteSignal } from "@maverick-js/signals";

import type { SignalId, SignalRegistry } from "./signal";

/**
 * The `__context` object passed into every compiled expression. It is the only
 * way generated code can reach back into the runtime — keeping the surface
 * narrow makes the generated JS easy to audit and keeps non-context globals
 * inaccessible from inside `new Function`.
 */
export class Context {
	constructor(private readonly registry: SignalRegistry) {}

	signal(id: SignalId): WriteSignal<unknown> {
		return this.registry.handle(id);
	}

	get builtin() {
		return builtin;
	}
}

const builtin = {
	f64(v: number): f64 {
		return new f64(v);
	},
};

class f64 {
	constructor(private readonly v: number) {}

	add(other: f64): f64 {
		return new f64(this.v + other.v);
	}

	sub(other: f64): f64 {
		return new f64(this.v - other.v);
	}

	mul(other: f64): f64 {
		return new f64(this.v * other.v);
	}

	div(other: f64): f64 {
		return new f64(this.v / other.v);
	}

	toString(): string {
		return this.v.toString();
	}
}
