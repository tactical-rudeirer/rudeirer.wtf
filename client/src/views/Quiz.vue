<template>
  <b-jumbotron>
    <b-container>
      <rainbow-text>Willkommen zum offiziellen 'Was mag Rudeirer'-Quiz!</rainbow-text>
      <QuizDisplay :quiz="quiz" @send="sendResult()" @clear="tryAgain()"/>
      <highscore :highscore="highscore" class="highscore" />
    </b-container>
  </b-jumbotron>
</template>

<script lang="ts">
import Vue from 'vue';
import RainbowText from '@/components/RainbowText.vue';
import QuizDisplay from '@/components/quiz/QuizDisplay.vue';
import Highscore from '@/components/quiz/Highscore.vue';

export default Vue.extend({
  components: {
    RainbowText,
    QuizDisplay,
    Highscore,
  },
  data() {
    return {
      quiz: {
        finalScore: undefined as (number | undefined),
        questions: [
          {
            item: 'Mayonaise',
            score: 0,
            result: undefined as (number | undefined),
          },
          {
            item: 'Buttermilch',
            score: 1,
          },
          {
            item: 'Schweineschnitzel',
            score: 5,
          },
        ],
      },
      highscore: [
        {
          rank: 1,
          name: 'Ich',
          score: 100,
        },
        {
          rank: 2,
          name: 'Du',
          score: 80,
        },
        {
          rank: 3,
          name: 'Lappen',
          score: 14,
        },
      ],
    };
  },
  methods: {
    sendResult() {
      console.log(this.quiz);
      this.quiz.finalScore = 5;
      this.quiz.questions.forEach((q) => q.result = Math.floor(Math.random() * 10));
    },
    tryAgain() {
      this.quiz.finalScore = undefined;
      this.quiz.questions.forEach((q) => q.result = undefined);
    },
  },
});
</script>

<style lang="scss" scoped>
.highscore {
  margin-top: 80px;
}
</style>
