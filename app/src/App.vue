<template>
  <v-app>
    <v-app-bar color="deep-purple accent-4" dark app>
      <v-toolbar-title class="headline text-uppercase">
        <span>Voice</span>
        <span class="font-weight-light">of Oasis</span>
      </v-toolbar-title>
      <v-spacer></v-spacer>
      <v-btn text icon >
         <v-icon>mdi-export-variant</v-icon>
      </v-btn>
    </v-app-bar>

    <v-content>
      <div class="main">
        <router-view />
      </div>
    </v-content>
  </v-app>
</template>

<script>
import { mapActions } from "vuex";

export default {
name: "App",
  async created() {
    let contractAddress = this.$route.query.id || localStorage.getItem('contractAddress');
    if (contractAddress === 'undefined' || contractAddress === 'null') {
        contractAddress = undefined;
    }
    console.log('ADDRESS: ', contractAddress);

    if (contractAddress) {
      await this.loadService(contractAddress); // this.$route.query.id);
    } else {
      await this.deployService();

      // Extract and parse ballot ID
      let ballotID = await this.getBallotID();
      ballotID = `0x${Array.from(ballotID, byte =>
        `0${(byte & 0xff).toString(16)}`.slice(-2)
      ).join("")}`;

      localStorage.setItem('contractAddress', ballotID);

      this.$router.push({
        path: "/",
        query: {
          id: ballotID
        }
      });
    }
  },
  methods: {
    ...mapActions(["loadService", "deployService", "getBallotID"])
  }
};
</script>
