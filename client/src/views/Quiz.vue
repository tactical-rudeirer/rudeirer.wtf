<template>
<div>
  <b-jumbotron>
    <b-container>
      <rainbow-text>Willkommen zum offiziellen 'Was mag Rudeirer'-Quiz!</rainbow-text>
    </b-container>
  </b-jumbotron>
  <b-container>
    <QuizDisplay :quiz="quiz" :name="name" @send="sendResult()" @clear="tryAgain()"/>
    <highscore :highscore="highscore" class="highscore" />
  </b-container>
</div>
</template>

<script lang="ts">
import Vue from 'vue';
import RainbowText from '@/components/RainbowText.vue';
import QuizDisplay from '@/components/quiz/QuizDisplay.vue';
import Highscore from '@/components/quiz/Highscore.vue';

interface Question {
  item: string;
  score: number;
  result?: number;
}
interface Quiz {
  finalScore?: number;
  questions: Question[];
}
interface HighscoreItem {
  rank: number;
  name: string;
  score: number;
}

export default Vue.extend({
  components: {
    RainbowText,
    QuizDisplay,
    Highscore,
  },
  data() {
    return {
      quiz: {
        finalScore: 0,
        questions: Array.apply(null, Array(10)).map((idx) => ({
            item: ' ',
            score: 0,
            result: undefined,
        })),
      } as Quiz,
      highscore: Array.apply(null, Array(10)).map((idx) => ({
          rank: idx,
          name: ' ',
          score: 0,
        })) as HighscoreItem[],
      name: '',
    };
  },
  created() {
    this.fetchHighscore();
    this.fetchQuestions();
  },
  methods: {
    async fetchHighscore() {
      const response = await fetch('https://api.rudeirer.wtf/quiz/highscore/');
      const highscore = await response.json();
      if (response.ok && highscore.success) {
        // assume correct order...
        highscore.data.forEach((el: any, idx: number) => el.rank = idx + 1);
        this.highscore = highscore.data;
      }
    },
    async fetchQuestions() {
      const response = await fetch('https://api.rudeirer.wtf/quiz/');
      const items = await response.json();
      if (response.ok && items.success) {
        const quiz: Quiz = {
          questions: [],
          finalScore: 0, // include heere to make sure its reactive
        };
        for (const item of items.data) {
          quiz.questions.push({
            item,
            score: 0,
          });
        }
        this.quiz = quiz;
      }
    },
    async sendResult() {
      interface RespMap {
        [key: string]: number;
      }
      const quizResponse: RespMap = {};
      this.quiz.questions.forEach((q: Question) => {
        quizResponse[q.item] = q.score;
      });
      const responseMsg = {
        name: this.name,
        quiz: quizResponse,
      };
      const response = await fetch('https://api.rudeirer.wtf/quiz/', {
        method: 'post',
        body: JSON.stringify(responseMsg),
      });
      const respResult = await response.json();
      if (response.ok && respResult.success) {
        this.quiz.finalScore = respResult.data.score;
        this.quiz.questions.forEach((q: Question) => {
          q.result = respResult.data.quiz[q.item];
        });
      }
    },
    tryAgain() {
      this.quiz.finalScore = undefined;
      this.quiz.questions.forEach((q) => q.result = undefined);
      this.name = '';
      this.fetchQuestions();
    },
  },
});
</script>

<style lang="scss" scoped>
.highscore {
  margin-top: 80px;
}
</style>
