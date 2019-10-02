<template>
    <b-container>
        <h4>Bewerten Sie, ob Rudi die folgenden Dinge mag auf einer Skala von 0 (nein) zu 10 (ja)</h4>
        <table class="table table-striped table-hover">
            <thead><tr><th>Name</th><th>Score</th><th v-if="quiz.finalScore">Lösung</th></tr></thead>
            <tbody>
                <tr v-for="question in quiz.questions" :key="question.item" :class="[questionRowClass(question)]">
                    <td>{{question.item}}</td>
                    <td>
                        <select class="form-control" v-model="question.score" :disabled="question.result !== undefined">
                            <option v-for="i in 11" :value="i-1" :key="i-1">{{i-1}}</option>
                        </select>
                    </td>
                    <td v-if="question.result !== undefined">{{question.result}}</td>
                </tr>
            </tbody>
        </table>
        <div class="form-group" v-if="!quiz.finalScore">
            <label for="highscoreName" class="mb-3 mt-4">Name für Highscore (optional)</label>
            <input id="highscoreName" type="text" class="form-control mb-3" placeholder="Name">
            <button class="btn btn-primary" v-on:click="showResult()">Ergebnis anzeigen!</button>
        </div>
        <div class="points" v-if="quiz.finalScore">
            <RainbowText>Final Score: {{quiz.finalScore}}</RainbowText>
            <button class="btn btn-primary" v-on:click="tryAgain()">Nochmal versuchen!</button>
        </div>
    </b-container>
</template>

<script lang="ts">
import Vue from 'vue';
import RainbowText from '@/components/RainbowText.vue';
export default Vue.extend({
    components: {
        RainbowText,
    },
    props: {
        quiz: {
            type: Object,
            required: true,
        },
    },
    methods: {
        showResult() {
            this.$emit('send', this.quiz);
        },
        tryAgain() {
            this.$emit('clear', this.quiz);
        },
        questionRowClass(qu: {score: number, result: number}) {
            return qu.score === qu.result ? 'quiz-row-correct' : '';
        },
    },
});
</script>

<style lang="scss" scoped>
.quiz-row-correct {
    background-color: rgb(151, 233, 151) !important;
}
.quiz-row-correct:hover {
    background-color: rgb(101, 236, 101) !important;
}
</style>
