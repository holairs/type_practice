use crate::utils::get_phrase::get_new_phrase;

pub fn create_display() {
    let phrase = get_new_phrase().expect("Error al obtener la frase"); // Obtener la frase
    let mut matrix = vec![vec![' '; 30]; 20]; // Crear una matriz de 20 filas y 30 columnas con espacios

    let mut current_row = 0; // Fila actual en la matriz
    let mut current_col = 0; // Columna actual en la matriz

    for word in phrase.split_whitespace() {
        // Dividir la frase en palabras
        let word_length = word.len();

        if current_col + word_length > 30 {
            // Si la palabra no cabe en la fila actual
            if current_row + 1 < 20 {
                // Pasar a la siguiente fila si hay espacio
                current_row += 1;
                current_col = 0;
            } else {
                // Si no hay espacio, cortar (puedes manejar este caso como prefieras)
                println!("La frase es demasiado larga para la matriz.");
                break;
            }
        }

        // Colocar la palabra en la fila actual
        for (i, c) in word.chars().enumerate() {
            matrix[current_row][current_col + i] = c;
        }

        // Actualizar la columna actual (dejar un espacio después de la palabra)
        current_col += word_length + 1;
    }

    // Imprimir la matriz
    for row in &matrix {
        let line: String = row.iter().collect();
        //println!("{}", line);
        println!("{}", line)
    }
}
