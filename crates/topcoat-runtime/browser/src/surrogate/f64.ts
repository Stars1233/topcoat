export class f64 {
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
