const { MongoClient } = require("mongodb");

module.exports = class QuerySingleton {
	/** @type {MongoClient} */
	#pool = null;

	#databaseUrl = null;
	#databaseName = null;

	/**
     * @inheritdoc
     * @returns {QuerySingleton}
     */
	static singleton () {
		if (!QuerySingleton.module) {
			QuerySingleton.module = new QuerySingleton();
		}

		return QuerySingleton.module;
	}

	constructor (options = {}) {
		if (!options.mongoIp) {
			throw new Error({
				message: "MongoDB IP address not specified."
			});
		}
		else if (!options.mongoPort) {
			throw new Error({
				message: "MongoDB port not specified."
			});
		}

		this.#databaseUrl = `mongodb://${options.mongoIp}:${options.mongoPort}`;
        
		if (!options.mongoDbName) {
			throw new Error({
				message: "MongoDB database name not specified."
			});
		}

		this.#databaseName = options.mongoDbName;
		this.#pool = new MongoClient(this.#databaseUrl);

		this.connect();
	}

	async connect () {
		const logger = await app.Logger("db");
		this.logger = logger;
		
		await this.#pool.connect()
			.then(() => this.logger.info("Connected to MongoDB."))
			.catch(e => console.error(e));

		this.initListeners();
	}

	initListeners () {
		const pool = this.#pool;

		pool.on("serverHeartbeatFailed", () => {
			this.logger.error("MongoDB server heartbeat failed.");
		});

		pool.on("topologyOpening", () => {
			this.logger.info("MongoDB topology opening.");
		});

		pool.on("topologyClosed", () => {
			this.logger.error("MongoDB topology closed.");
		});
	}

	destroy () {
		this.#pool.close();
		this.#pool.removeAllListeners();

		this.#pool = null;
	}

	client () { return this.#pool.db(this.#databaseName); }

	get modulePath () { return "query"; }
};
