# pyro

## Concepto
Es una aplicación de terminal (basicamente un CLI) que le pasas un archivo en algun formato y una palabra a buscar. Cuando la encuentre, está va a estar resaltada en amarillo. La palabra puede estar en minusculas como mayusculas que igual la va a reconocer y mostrar en pantalla (junto a la linea donde fue encontrada)

## Requisitos
- Tener cargo para compilar el programa

## Uso
No seas boludo, no ejecutes el .exe porque asi no funciona.
- Cloná el repo
- Hacé un
  
´´´
cargo build --release
´´´

- Abris una terminal en ese directorio (target/release/), escribis el nombre de la aplicación, le pasas los argumentos (archivo y palabra para buscar) y listo.
### Para agregar al $PATH de modo que te funcione en todo el sistema
- En linux depende la shell, pero mete un echo $PATH y fijate si tu carpeta .local/bin/ esta dentro, sino podes meterla en /usr/local que tambien va; usá ln -s para armar un link simbólico y enlazalo a tu ejecutable que está dentro del repo, si es que queres rebuildear con cambios de forma facil
- En windows haces lo siguiente
   - Te vas al disco C (o al disco donde tengas tus archivos de programa)
   - Creas una carpeta dentro de archivos de programa que se llame como la aplicacion (o como te pinte)
   - Metes el ejecutable de la carpeta release ahi dentro
   - En el buscar de windows ponés editar variables de entorno
   - Apretás donde dice variables de entorno y vas a ver una parte que dice $PATH, haces doble click ahi (o editar)
   - Apretás el boton que dice nuevo, y ponés el link de la carpeta donde metiste el ejecutable de hoy
   - Reiniciá la terminal y listo
