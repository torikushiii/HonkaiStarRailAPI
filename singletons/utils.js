const { load } = require("cheerio");

module.exports = class UtilsSingleton {
	/**
	 * @inheritdoc
	 * @returns {UtilsSingleton}
	 */
	static singleton () {
		if (!UtilsSingleton.module) {
			UtilsSingleton.module = new UtilsSingleton();
		}

		return UtilsSingleton.module;
	}

	cheerio (html) {
		return load(html);
	}
};
