<template>
<div>
    <b-jumbotron>
        <b-container>
            <rainbow-text>Willkommen auf der LeibiBörse</rainbow-text> 
            <b-container v-if="isLoggedIn" class="balance-container">
                <h3>Eigene Balance</h3>
                <table class="table table-striped table-hover">
                    <thead><tr><th>Währung</th><th>Menge</th><th>Market cap</th></tr></thead>
                    <tbody>
                        <tr v-for="b in balance" :key="b.symbol">
                            <td>{{b.currency}} ({{b.symbol}})</td>
                            <td>{{b.amount}}</td>
                            <td>{{marketCapFor(b.symbol, b.amount)}}% of market cap</td>
                        </tr>
                    </tbody>
                </table>
                <h4 style="margin-top: 60px;">Transaktionen</h4>
                <table class="table table-striped table-hover">
                    <thead><tr><th>Nutzer</th><th>Betrag</th><th>Verwendungszweck</th><th>Datum</th></tr></thead>
                    <tbody>
                        <tr v-for="(t, idx) in transactions" :key="idx">
                            <td>{{t.type === 'received' ? t.sender : t.sender}}</td>
                            <td>
                                <span :class="[t.type === 'received' ? 'm-green' : 'm-red']">
                                    {{t.type === 'received' ? '+' : '-'}}{{t.amount}} {{t.symbol}}
                                </span>
                            </td>
                            <td>{{t.reference}}</td>
                            <td>{{formatTransactionDate(t.time)}}</td>
                        </tr>
                    </tbody>
                </table>
            </b-container>
            <b-container v-else class="balance-container">
                <b-link to="/login">Bitte loggen Sie sich ein um eine Übersicht über ihre Balance zu sehen</b-link>
            </b-container>
        </b-container>
    </b-jumbotron>
    <b-container>
        <h3>Kurse</h3>
        <price class="card shadow price-card" :price="c" v-for="c in currencies" :key="c.symbol"/>
    </b-container>
</div>
</template>

<script lang="ts">
import Vue from 'vue';
import RainbowText from '@/components/RainbowText.vue';
import Price from '@/components/stocks/Price.vue';
export default Vue.extend({
    components: {
        RainbowText,
        Price,
    },
    data() {
        return {
            isLoggedIn: true,
            balance: [
                {
                    currency: 'RudiCoins',
                    symbol: 'rudicoins',
                    amount: 42,
                },
                {
                    currency: 'Jens',
                    symbol: 'jens',
                    amount: 0.08,
                },
                {
                    currency: 'Leibi',
                    symbol: 'leibi',
                    amount: 3,
                },
            ],
            currencies: [
                {
                    currency: 'RudiCoins',
                    symbol: 'rudicoins',
                    total: 500,
                    rudicoinRate: 12,
                    history: [],
                    date: new Date(),
                },
                {
                    currency: 'Jens',
                    symbol: 'jens',
                    total: 1,
                    rudicoinRate: 500,
                    history: [],
                    date: new Date(),
                },
                {
                    currency: 'Leibi',
                    symbol: 'leibi',
                    total: 50,
                    rudicoinRate: 500,
                    history: [],
                    date: new Date(),
                },
            ],
            transactions: [
                {
                    sender: 'Magic Rudicoin Source',
                    recipient: 'Temp',
                    amount: 5,
                    currency: 'Rudicoin',
                    symbol: 'rudicoin',
                    reference: 'Quiz',
                    type: 'received',
                    time: new Date(),
                },
                {
                    sender: 'Ich',
                    recipient: 'Rudi',
                    amount: 20,
                    currency: 'Leibi',
                    symbol: 'leibi',
                    reference: 'verkauft an rudi',
                    type: 'spend',
                    time: new Date(),
                },
            ],
        };
    },
    methods: {
        marketCapFor(symbol: string, amount: number) {
            const currency = this.currencies.find((c) => c.symbol === symbol);
            if (currency) {
                return amount / currency.total;
            } else {
                return 'market cap nicht verfügbar';
            }
        },
        formatTransactionDate(d: Date) {
            return d.toLocaleDateString('de-DE', {
                year: '2-digit', month: '2-digit', day: '2-digit',
                hour: '2-digit', minute: '2-digit', second: '2-digit',
            });
        },
    },
});
</script>

<style lang="scss">
.balance-container {
    margin-top: 30px;
}
.price-card {
    margin-top: 30px;
}
.m-green {
    color: green;
}
.m-red {
    color: red;
}

</style>


