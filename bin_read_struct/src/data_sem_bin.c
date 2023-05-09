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
    FILE *fp = fopen("data_sem.bin", "wb");

    ValueStruct data[100];

    // Verifichiamo che il file sia stato aperto correttamente
    if (fp == NULL) {
        printf("Errore nell'apertura del file\n");
        return 1;
    }

    // Inizializziamo il generatore di numeri casuali
    srand(time(NULL));

    // Generiamo 100 struct casuali e le scriviamo su file
    for (int i = 0; i < 100; i++) {
        int tmp_type;
        float tmp_val;
        long tmp_timestamp;
        tmp_type = rand() % 10; // type sarà 0 o 9
        tmp_val = ((float) rand()) / RAND_MAX; // val sarà un float tra 0 e 1
        tmp_timestamp = time(NULL) + i; // timestamp sarà l'ora corrente più un offset di i secondi
        data[i].type = tmp_type;
        data[i].val = tmp_val;
        data[i].val = tmp_val;
        data[i].timestamp = tmp_timestamp;
        fwrite(&tmp_type, sizeof(int), 1, fp);
        fwrite(&tmp_val, sizeof(float), 1, fp);
        fwrite(&tmp_timestamp, sizeof(long), 1, fp);
    }
    for(int i = 0; i < 100; i++) {
        printf("%d %f %ld\n", data[i].type, data[i].val, data[i].timestamp);
    }

    printf("Sizeof int: %d\n", (int)sizeof(int));
    printf("Sizeof long: %d\n", (int)sizeof(long));
    printf("Sizeof float: %d\n", (int)sizeof(float));
    
    // Chiudiamo il file
    fclose(fp);

    return 0;
}