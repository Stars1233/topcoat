import type { WriteSignal as MaverickWriteSignal } from "@maverick-js/signals";

import { Ref } from "./ref";

export class WriteSignal<T> {
	constructor(private readonly inner: MaverickWriteSignal<Ref<T>>) {}

	read(): Ref<T> {
		return this.inner();
	}

	set(v: T): void {
		this.inner.set(new Ref(v));
	}
}
