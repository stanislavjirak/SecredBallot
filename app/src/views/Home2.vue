<!--  <HelloWorld />-->

<template>
    <v-card
        flat
        class="py-12"
    >
        <v-card-text>
            <v-row
                align="center"
                justify="center"
                v-for="(option, index) of options">
                <v-col cols="12">
                    <p class="text-center">Mandatory</p>
                </v-col>
                <v-btn-toggle
                    v-model="options[index]"
                    mandatory
                >
                    <v-btn>
                        <v-icon>mdi-format-align-left</v-icon>
                    </v-btn>
                    <v-btn>
                        <v-icon>mdi-format-align-center</v-icon>
                    </v-btn>
                    <v-btn>
                        <v-icon>mdi-format-align-right</v-icon>
                    </v-btn>
                    <v-btn>
                        <v-icon>mdi-format-align-justify</v-icon>
                    </v-btn>
                </v-btn-toggle>
            </v-row>

            <v-btn @click="setModified()">CLICK</v-btn>
        </v-card-text>
    </v-card>
</template>

<script>
    // import HelloWorld from '../components/HelloWorld';

    export default {
        data() {
            return {
                options: [
                    0, 1, 2
                ],
                toggle_exclusive: 1,
            }
        },
        methods: {
            setModified() {
                console.log('DATA: ', this.options);
            }
        }
    }


    // export default {
    //   components: {
    //     HelloWorld,
    //   },
    // };
</script>
