#include <stdio.h>
#include <stdlib.h>
#include <time.h>

typedef struct {
    int type;
    float val;
    long timestamp;
} ValueStruct;

int main() {
    // Apriamo il file in scrittura
    FILE *fp = fopen("valori.txt", "w");

    // Verifichiamo che il file sia stato aperto correttamente
    if (fp == NULL) {
        printf("Errore nell'apertura del file\n");
        return 1;
    }

    // Inizializziamo il generatore di numeri casuali
    srand(time(NULL));

    // Generiamo 100 struct casuali e le scriviamo su file
    for (int i = 0; i < 100; i++) {
        ValueStruct val;
        val.type = rand() % 2; // type sarà 0 o 1
        val.val = ((float) rand()) / RAND_MAX; // val sarà un float tra 0 e 1
        val.timestamp = time(NULL) + i; // timestamp sarà l'ora corrente più un offset di i secondi

        fprintf(fp, "%d %f %ld\n", val.type, val.val, val.timestamp);
    }

    // Chiudiamo il file
    fclose(fp);

    return 0;
}
