const { CronJob } = require("cron");

module.exports = class Cron extends require("./template") {
	name;
	expression;
	code;
	data;

	static importable = true;

	constructor (data) {
		super();

		this.name = data.name;
		if (typeof this.name !== "string") {
			throw new app.Error({
				message: "Cron name must be a string",
				args: {
					name: data
				}
			});
		}

		this.expression = data.expression;
		if (typeof this.expression !== "string") {
			throw new app.Error({
				message: "Cron expression must be a string",
				args: {
					expression: data
				}
			});
		}

		if (typeof data.code === "function") {
			this.code = data.code.bind(this);
		}
		else {
			throw new app.Error({
				message: "Cron code must be a function",
				args: {
					code: data
				}
			});
		}

		this.job = null;
		this.data = {};
	}

	start () {
		if (this.started) {
			return this;
		}

		if (!this.expression) {
			console.error(`Cron ${this.name} has no expression`);
			return this;
		}

		this.job = new CronJob(this.expression, () => this.code());

		this.job.start();
		this.started = true;

		return this;
	}

	stop () {
		if (!this.started) {
			return this;
		}

		if (!this.job) {
			throw new app.Error({
				message: `Cron ${this.name} has no job`
			});
		}

		this.job.stop();
		this.started = false;
		return this;
	}

	destroy () {
		if (this.job && this.started) {
			this.stop();
		}

		this.job = null;
	}

	static async initialize () {
		this.data = [];
		return this;
	}

	static async importData (definitions) {
		super.importData(definitions);

		for (const cron of Cron.data) {
			cron.start();
		}
	}
};
