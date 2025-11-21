SEPERATOR=$(echo -ne '\n')

clear

echo $SEPERATOR

echo -ne "\n\t-----------------| Билдер Salarixi Onion |-----------------\n\n"

echo $SEPERATOR

read -p "Укажите путь к проекту (t - текущий): " BUILD_PATH && [[ $BUILD_PATH == [tT] ]] || cd $BUILD_PATH

echo $SEPERATOR

echo "( check ) Проверка дерикторий..."

PROJECT_CORE_PATH="$(pwd)/project/core"
PROJECT_interface_PATH="$(pwd)/project/interface"
PROJECT_UTILS_PATH="$(pwd)/project/utils"
PROJECT_CLI_PATH="$(pwd)/project/cli"

echo $SEPERATOR

if [ -d $PROJECT_CORE_PATH ]; then
  echo "( check ) Ядро проекта (core) => Найдено"
else
  echo "( check ) Ядро проекта (core) => Не удалось найти"; exit 1
fi

if [ -d $PROJECT_interface_PATH ]; then
  echo "( check ) Интерфейс проекта (interface) => Найдено"
else
  echo "( check ) Интерфейс проекта (interface) => Не удалось найти"; exit 1
fi

if [ -d $PROJECT_UTILS_PATH ]; then
  echo "( check ) Утилиты проекта (utils) => Найдено"
else
  echo "( check ) Утилиты проекта (utils) => Не удалось найти"; exit 1
fi

if [ -d $PROJECT_CLI_PATH ]; then
  echo "( check ) Терминальные утилиты проекта (cli) => Найдено"
else
  echo "( check ) Терминальные утилиты проекта (cli) => Не удалось найти"; exit 1
fi
 
echo $SEPERATOR

echo "( preparation ) Подготовка..."

USER=$(whoami)
CURRENT_DIR=$(pwd)

echo "( info ) Пользователь: $USER"
echo "( info ) Текущая дериктория: $CURRENT_DIR"
echo "( info ) Дериктория проекта: $CURRENT_DIR"

echo "( preparation ) Подготовка окончена"

echo $SEPERATOR

read -p "Удалить старые сборки? (y/N): " confirm && [[ $confirm == [yY] || $confirm == [yY][eE][sS] ]] || exit 0

echo "( remove ) Удаление старых сборок..."

echo "( remove / core ) Удаление ядра..."

rm $CURRENT_DIR/dev/build/services/so-core > /dev/null 2>&1

echo "( remove / interface ) Удаление интерфейса..."

rm $CURRENT_DIR/dev/build/services/so-interface > /dev/null 2>&1

echo "( remove / utils ) Удаление утилит..."

rm $CURRENT_DIR/dev/build/services/so-utils > /dev/null 2>&1
rm -rf $CURRENT_DIR/dev/build/config > /dev/null 2>&1

echo "( remove / cli ) Удаление терминальных утилит..."

rm -rf $CURRENT_DIR/dev/build/services/cli > /dev/null 2>&1

echo "( remove / loader ) Удаление загрузчика..."

rm $CURRENT_DIR/dev/build/salarixi-loader > /dev/null 2>&1

# echo "( remove / archive ) Удаление архива..."
# rm -rf $CURRENT_DIR/dev/salarixionion.zip > /dev/null 2>&1
# echo "( remove ) Удаление завершено"

echo $SEPERATOR

echo "( generation / core ) Генерация JavaScript-кода..."

cd $CURRENT_DIR/project/core

npm run generate > /dev/null

echo "( generation / core ) Генерация JavaScript-кода прошла успешно"
echo "( generation / core ) Путь к сгенерированному коду: $(pwd)/dist"

echo $SEPERATOR

echo "( build / windows / core ) Сборка исполняющегося файла (core)..."

npx pkg -t node18-win-x86_64 -o $CURRENT_DIR/dev/build/services/so-core ./dist/index.js > /dev/null

echo "( build / windows / core ) Сборка окончена"
echo "( build / windows / core ) Путь к исполнимому файлу: $CURRENT_DIR/dev/build/services/so-core"

echo $SEPERATOR

# echo "( compression / core ) Сжатие исполняющегося файла..."
# cd $CURRENT_DIR
# upx --no-reloc --preserve-build-id -1 ./dev/build/core > /dev/null
# echo "( compression / core ) Исполняющийся файл успешно сжат"
# echo $SEPERATOR

echo "( build / windows / interface ) Сборка исполняющегося файла (interface)..."

cd $CURRENT_DIR/project/interface

npm run tauri build > /dev/null

mv ./src-tauri/target/release/salarixionion $CURRENT_DIR/dev/build/services/so-interface

echo "( build / windows / interface ) Сборка окончена"
echo "( build / windows / interface ) Путь к исполнимому файлу: $CURRENT_DIR/dev/build/services/so-interface"

echo $SEPERATOR

echo "( build / windows / utils ) Сборка исполняющегося файла (utils)..."

cd $CURRENT_DIR/project/utils/src

go build utils > /dev/null

mv ./utils $CURRENT_DIR/dev/build/services/so-utils

mkdir $CURRENT_DIR/dev/build/config

cp ../config/salarixi.config.json $CURRENT_DIR/dev/build/config/salarixi.config.json

echo "( build / windows / utils ) Сборка окончена"
echo "( build / windows / utils ) Путь к исполнимому файлу: $CURRENT_DIR/dev/build/services/so-utils"

echo $SEPERATOR

echo "( build / windows / cli ) Сборка исполняющегося файла (cli / runner)..."

mkdir $CURRENT_DIR/dev/build/services/cli
mkdir $CURRENT_DIR/dev/build/services/cli/tools

cd $CURRENT_DIR/project/cli/src

pyinstaller --onefile runner.py > /dev/null 2>&1

mv ./dist/runner $CURRENT_DIR/dev/build/services/cli/runner

echo "( build / cli ) Сборка исполняющегося файла (cli / crasher)..."

cd $CURRENT_DIR/project/cli/src/crasher

pyinstaller --onefile crasher.py > /dev/null 2>&1

mv ./dist/crasher $CURRENT_DIR/dev/build/services/cli/tools/crasher

echo "( build / cli ) Сборка окончена"
echo "( build / cli ) Путь к исполнимым файлам: $CURRENT_DIR/dev/build/services/cli"

echo $SEPERATOR

echo "( build / loader ) Создание загрузчика..."

cd $CURRENT_DIR/project/loader

dotnet publish -r win-x64 -c Release --self-contained -o build ./src/Program.cs > /dev/null

mv ./build/Program $CURRENT_DIR/dev/build/salarixi-loader

echo "( build / loader ) Сборка окончена"
echo "( build / loader ) Путь к исполнимому файлу: $CURRENT_DIR/dev/build/salarixi-loader"

# echo "( archive ) Создание ZIP-архива..."
# cd $CURRENT_DIR/dev
# zip -r salarixionion.zip ./build > /dev/null
# echo "( archive ) ZIP-архив успешно создан"