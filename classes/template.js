module.exports = class ClassTemplate {
	static importable = false;

	static uniqueIdentifier;

	destroy () {}

	static data = [];

	static importData (definitions) {
		if (!this.importable) {
			throw new app.Error({
				message: "This class is not importable"
			});
		}

		if (this.data && this.data.length !== 0) {
			for (const instance of this.data) {
				instance.destroy();
			}

			this.data = [];
		}

		this.data = definitions.map((definition) => new this(definition));
	}
};
