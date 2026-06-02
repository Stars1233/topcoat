// biome-ignore lint/suspicious/noShadowRestrictedNames: Surrogate type
export class String {
	constructor(private readonly v: string) {}

	clone(): String {
		return new String(this.v);
	}

	toJSON(): string {
		return this.v;
	}

	toString(): string {
		return this.v.toString();
	}
}
