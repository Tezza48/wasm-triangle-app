var BaseStats = require("stats.js")

export class Stats {
    constructor() {
        this.stats = BaseStats();
    }

    get dom() {
        return this.stats.dom;
    }

    update() {
        this.stats.update()
    }
}