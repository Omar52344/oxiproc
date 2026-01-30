#!/bin/bash
# Instalador simple para Oxiproc en Linux
# Instala el binario en ~/.local/bin

APP_NAME="oxiproc"
INSTALL_DIR="$HOME/.local/bin"
SOURCE_BIN="./$APP_NAME"

# Colores
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo "Iniciando instalación de $APP_NAME..."

# 1. Verificar que el binario existe
if [ ! -f "$SOURCE_BIN" ]; then
    echo -e "${RED}Error: No se encontró el archivo '$APP_NAME'.${NC}"
    echo "Asegúrate de que el ejecutable (sin extensión .exe) esté en la misma carpeta que este script."
    exit 1
fi

# 2. Crear directorio si no existe
if [ ! -d "$INSTALL_DIR" ]; then
    mkdir -p "$INSTALL_DIR"
    echo -e "${GREEN}Directorio $INSTALL_DIR creado.${NC}"
fi

# 3. Copiar y dar permisos
cp "$SOURCE_BIN" "$INSTALL_DIR/"
chmod +x "$INSTALL_DIR/$APP_NAME"
echo -e "${GREEN}$APP_NAME instalado exitosamente en $INSTALL_DIR${NC}"

# 4. Verificar PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo -e "${YELLOW}NOTA IMPORTANTE:${NC}"
    echo -e "El directorio $INSTALL_DIR no parece estar en tu PATH."
    echo "Para ejecutar $APP_NAME desde cualquier lugar, agrega esto a tu .bashrc o .zshrc:"
    echo ""
    echo "    export PATH=\"\$HOME/.local/bin:\$PATH\""
    echo ""
else
    echo -e "${GREEN}¡Listo! Ya puedes ejecutar '$APP_NAME' desde tu terminal.${NC}"
fi
