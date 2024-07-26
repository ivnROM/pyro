# pyro

## Concepto
Es una aplicación de terminal (basicamente un CLI) que le pasas un archivo en algun formato y una palabra a buscar. Cuando la encuentre, está va a estar resaltada en amarillo. La palabra puede estar en minusculas como mayusculas que igual la va a reconocer y mostrar en pantalla (junto a la linea donde fue encontrada)

## Uso
No seas boludo, no ejecutes el .exe porque asi no funciona.
- Cloná el repo
- Abri la carpeta target/release donde va aestar tu ejecutable
- Abris una terminal en ese directorio, escribis el nombre de la aplicación, le pasas los argumentos (archivo y palabra para buscar) y listo.
### Para agregar al $PATH de modo que te funcione en todo el sistema
- Fijate q onda linux, yo todavia no me fije, capaz con un export ya funca
- En windows haces lo siguiente
   - Te vas al disco C (o al disco donde tengas tus archivos de programa)
   - Creas una carpeta dentro de archivos de programa que se llame como la aplicacion (o como te pinte)
   - Metes el ejecutable de la carpeta release ahi dentro
   - En el buscar de windows ponés editar variables de entorno
   - Apretás donde dice variables de entorno y vas a ver una parte que dice $PATH, haces doble click ahi (o editar)
   - Apretás el boton que dice nuevo, y ponés el link de la carpeta donde metiste el ejecutable de hoy
   - Reiniciá la terminal y listo
