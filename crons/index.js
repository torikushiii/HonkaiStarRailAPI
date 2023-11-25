const CodeRedemptionScraper = require("./code-redemption-scraper");
const CodeValidator = require("./code-validator");
const HoyoLabNews = require("./hoyolab-news");

const definitions = [
	CodeRedemptionScraper,
	CodeValidator,
	HoyoLabNews
];

module.exports = definitions;
