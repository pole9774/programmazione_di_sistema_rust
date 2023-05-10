#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

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
    int type; // 1=Value 2=MValue 3=Message
    union {
        ValueStruct val;
        MValueStruct mvals;
        MessageStruct messages;
    };
} ExportData;

int main() {
    int i;
    ExportData data[100];
    srand(time(NULL)); // inizializza il generatore di numeri casuali con il tempo attuale
    FILE *fp = fopen("data.txt", "w"); // apre il file in modalità scrittura binaria
    if (fp == NULL) {
        printf("Errore: impossibile aprire il file\n");
        return 1;
    }
    for (i = 0; i < 100; i++) {
        int r = rand() % 3; // sceglie casualmente il tipo di dato da generare (0=Value, 1=MValue, 2=Message)
        data[i].type = r + 1;
        fprintf(fp, "%d ", data[i].type);
        switch (r) {
            case 0: // ValueStruct
                data[i].val.type = 1;
                data[i].val.val = (float)(rand() % 10000) / 100.0f; // genera un valore casuale tra 0 e 100.00
                data[i].val.timestamp = time(NULL);
                fprintf(fp, "%d %f %ld\n", data[i].val.type, data[i].val.val, data[i].val.timestamp);
                break;
            case 1: // MValueStruct
                data[i].mvals.type = 2;
                int j;
                for (j = 0; j < 10; j++) {
                    data[i].mvals.val[j] = (float)(rand() % 10000) / 100.0f; // genera 10 valori casuali tra 0 e 100.00
                }
                data[i].mvals.timestamp = time(NULL);
                fprintf(fp, "%d ", data[i].mvals.type);
                for (j = 0; j < 10; j++) {
                    fprintf(fp, "%f ", data[i].mvals.val[j]);
                }
                fprintf(fp, "%ld\n", data[i].mvals.timestamp);
                break;
            case 2: // MessageStruct
                data[i].messages.type = 3;
                snprintf(data[i].messages.message, 21, "Message %d", i); // genera un messaggio con una stringa diversa per ogni elemento
                fprintf(fp, "%d %s\n", data[i].messages.type, data[i].messages.message);
                break;
        }
    }
    fclose(fp); // chiude il file
    return 0;
}
