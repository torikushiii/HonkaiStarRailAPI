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

	languageCodeConverter (lang) {
		switch (lang) {
			case "en":
				return "en-us";
			case "cn":
				return "zh-cn";
			case "tw":
				return "zh-tw";
			case "de":
				return "de-de";
			case "es":
				return "es-es";
			case "fr":
				return "fr-fr";
			case "id":
				return "id-id";
			case "it":
				return "it-it";
			case "jp":
				return "ja-jp";
			case "kr":
				return "ko-kr";
			case "pt":
				return "pt-pt";
			case "ru":
				return "ru-ru";
			case "th":
				return "th-th";
			case "tr":
				return "tr-tr";
			case "vn":
				return "vi-vn";
			default:
				return "en-us";
		}
	}
};
