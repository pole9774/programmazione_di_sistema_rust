#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

// definizione delle struct
typedef struct {
    int type;
    float val;
    long timestamp;
} ValueStruct;

typedef struct {
    int type;
    float val[10];
    long timestamp;
} MValueStruct;

typedef struct {
    int type;
    char message[21];
} MessageStruct;

typedef struct {
    int type;
    union {
        ValueStruct val;
        MValueStruct mvals;
        MessageStruct messages;
    };
} ExportData;

int main() {
    // creazione del vettore di ExportData
    ExportData data[100];

    srand(time(NULL));
    int i;

    // apertura del file in scrittura binaria
    FILE *fp = fopen("export.bin", "wb");
    if (fp == NULL) {
        printf("Errore nell'apertura del file\n");
        return 1;
    }

    for (i = 0; i < 100; i++) {
        int dataType = rand() % 3 + 1; // scelta casuale del tipo di dato
        data[i].type = dataType;
        fwrite(&dataType, sizeof(int), 1, fp);
        printf("%d ", dataType);
        switch (dataType) {
            case 1: {
                int tmp_type = dataType;
                float tmp_val = ((float) rand() / RAND_MAX) * 100.0f; // valore casuale tra 0 e 100
                long tmp_timestamp = time(NULL);
                data[i].val.type = tmp_type;
                data[i].val.val = tmp_val;
                data[i].val.timestamp = tmp_timestamp;
                fwrite(&tmp_type, sizeof(int), 1, fp);
                fwrite(&tmp_val, sizeof(float), 1, fp);
                fwrite(&tmp_timestamp, sizeof(long), 1, fp);
                printf("(%d ", tmp_type);
                printf("%f ", tmp_val);
                printf("%ld)\n", tmp_timestamp);
                break;
            }
            case 2: {
                int tmp_type = dataType;
                int j;
                float tmp_val[10];
                for (j = 0; j < 10; j++) {
                    tmp_val[j] = ((float) rand() / RAND_MAX) * 100.0f; // valore casuale tra 0 e 100
                }
                long tmp_timestamp = time(NULL);
                data[i].mvals.type = tmp_type;
                for (j = 0; j < 10; j++) {
                    data[i].mvals.val[j] = tmp_val[j];
                }
                data[i].mvals.timestamp = tmp_timestamp;
                fwrite(&tmp_type, sizeof(int), 1, fp);
                for (j = 0; j < 10; j++) {
                    fwrite(&tmp_val[j], sizeof(float), 1, fp);
                }
                fwrite(&tmp_timestamp, sizeof(long), 1, fp);
                printf("(%d [", tmp_type);
                for (j = 0; j < 10; j++) {
                    printf("%f ", tmp_val[j]);
                }
                printf("] %ld)\n", tmp_timestamp);
                break;
            }
            case 3: {
                int tmp_type = dataType;
                // generazione di una stringa casuale di lunghezza tra 1 e 20
                int msgLen = 20;
                int j;
                char tmp_message[21];
                for (j = 0; j < msgLen; j++) {
                    tmp_message[j] = 'a' + rand() % 26;
                }
                tmp_message[msgLen] = '\0';
                data[i].messages.type = tmp_type;
                for (j = 0; j < msgLen; j++) {
                    data[i].messages.message[j] = tmp_message[j];
                }
                fwrite(&tmp_type, sizeof(int), 1, fp);
                for (j = 0; j < 21; j++) {
                    fwrite(&tmp_message[j], sizeof(char), 1, fp);
                }
                printf("(%d ", tmp_type);
                for (j = 0; j < 21; j++) {
                    printf("%c", tmp_message[j]);
                }
                printf(")\n");
                break;
            }
        }
    }
    
    // chiusura del file
    fclose(fp);
    
    return 0;
}
