<template>
  <v-container id="Vote_Container">
    <v-card >
      <v-list-item three-line>
        <v-list-item-content class="align-self-start">
          <v-list-item-title class="headline mb-2">{{ question }}</v-list-item-title>
          <v-list-item-subtitle>Help us decide</v-list-item-subtitle>
        </v-list-item-content>

        <v-list-item-avatar size="125">
          <v-img src="/assets/prague.jpg"></v-img>
        </v-list-item-avatar>
      </v-list-item>
    </v-card>
<div v-if="loading">
    <v-progress-linear indeterminate color="yellow darken-2" ></v-progress-linear>
</div>

    <v-list-item-group v-model="radios" multiple>
      <v-row v-for="(option, index) of options" :key="options[index]">
        <v-col cols="12">
          <p>{{ originalopts[index] }}</p>
          <v-btn-toggle v-model="options[index]">
            <v-btn text value="0">
              <span class="hidden-sm-and-down">Downvote</span>
              <v-icon>thumb_down</v-icon>
            </v-btn>
            <v-btn text value="1" disabled>
              <span class="hidden-sm-and-down">No vote</span>
            </v-btn>
            <v-btn text value="2">
              <span class="hidden-sm-and-down">+1 Vote</span>
              <v-icon>thumb_up</v-icon>
            </v-btn>
            <v-btn text value="3">
              <span class="hidden-sm-and-down">+2 Vote</span>
              <v-icon color="green">thumb_up</v-icon>
            </v-btn>
          </v-btn-toggle>
        </v-col>
      </v-row>
    </v-list-item-group>

    <br />

    <div class="text-xs-center" v-if="!loading">
      <v-btn id="Vote_SelectedButton" @click="submit">Submit your vote</v-btn>
    </div>
    <div id="Vote_Warning" class="pt-3" v-if="!loading">
      <img id="Vote_WarningIcon" src="/assets/SecretBallot_warn.svg" />
      <span class="pl-1">
        Once you submit, you won't be able to change your answer
        or vote again as a different identity.
      </span>
    </div>
  </v-container>
</template>

<script>
import { mapActions } from "vuex";

export default {
  name: "Vote",
  async created() {
    if (!this.$route.query.id) {
      return;
    }
    await this.loadService(this.$route.query.id);

    // Check if the vote is closed, and and redirect to results page if it is
    const open = await this.getOpen();
    if (!open) {
      this.$router.push({ name: "results", query: this.$route.query });
      return;
    }

    this.question = await this.getDescription();
    this.options = await this.getCandidates();
    this.originalopts = await this.getCandidates();
    this.loading = false;
  },
  data() {
    return {
      loading: true,
      options: [],
      originalopts:[],
      question: "Loading questions from blockchain",
      radios: []
    };
  },
  methods: {
    ...mapActions([
      "loadService",
      "castVote",
      "getDescription",
      "getCandidates",
      "getOpen"
    ]),
    async submit() {
      this.loading = true;
      const options = this.options.map(item => parseInt(item));
      const three = options.findIndex(item => item === 3);
      const two = options.findIndex(item => item === 2);
      const minusOne = options.findIndex(item => item === 0);

      const isDefined = num => {
        return typeof num === "number" && !Number.isNaN(num);
      };

      const toSend = [
        isDefined(three) ? three : -1,
        isDefined(two) ? two : -1,
        isDefined(minusOne) ? minusOne : -1
      ];
      await this.castVote(toSend);
      console.log(this.radios);

      this.loading = false;
      this.$router.push({ name: "confirm", query: this.$route.query });
    }
  }
};
</script>

