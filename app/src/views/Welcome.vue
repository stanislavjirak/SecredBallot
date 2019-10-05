<template>
  <v-container id="Welcome_Container" grid-list-md fill-height>
  <v-layout row wrap>
    
    <v-card class="mx-auto" max-width="400">
      <v-img class="white--text" height="200px" src="/assets/prague.jpg">
        <v-card-title class="align-end fill-height">Improve Prague!!!</v-card-title>
      </v-img>

      <v-card-text>
        <span>Improve living in Prague</span>
        <br />
        <span class="text--primary">
          <span>Help us decide on the next step</span>
        </span>
      </v-card-text>

      <v-card-actions>
        <v-btn
          id="Welcome_Button"
          :loading="!ballot"
          @click="$router.push({ name: 'vote', query: $route.query })"
          text
          color="orange"
        >Participate</v-btn>
      </v-card-actions>
    </v-card>
    <v-card class="mx-auto" max-width="400">
      <v-img class="white--text" height="200px" src="/assets/dogs.jpg">
        <v-card-title class="align-end fill-height">Dogs</v-card-title>
      </v-img>

      <v-card-text>
        <span>Do you want dogs in downtown?</span>
        <br />
        <span class="text--primary">
          <span>Decide on fate of your four legged friends</span>
        </span>
      </v-card-text>

      <v-card-actions>
        <v-btn disabled>Participate</v-btn>
      </v-card-actions>
    </v-card>
    <v-card class="mx-auto" max-width="400">
      <v-img class="white--text" height="200px" src="/assets/egovernment.jpg">
        <v-card-title class="align-end fill-height">eGovernment</v-card-title>
      </v-img>

      <v-card-text>
        <span>What next eG tech do you want to implement?</span>
        <br />
        <span class="text--primary">
          <span>What is the best next step?</span>
        </span>
      </v-card-text>

      <v-card-actions>
        <v-btn disabled>Participate</v-btn>
      </v-card-actions>
    </v-card>
  </v-layout>
  </v-container>
</template>

<script>
import { mapState } from "vuex";

export default {
  name: "Welcome",
  computed: {
    ...mapState(["ballot"])
  }
};
</script>