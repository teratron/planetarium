# Строки главного меню
# Этот файл является частью многоразового шаблона лаунчера.
menu-title = PLANETARIUM
menu-play = Играть
menu-settings = Настройки
menu-exit = Выход
menu-back = Назад

# Вкладки настроек
settings-title = Настройки
tab-graphics = Графика
tab-audio = Звук
tab-controls = Управление
tab-general = Общие

# Настройки графики
setting-resolution = Разрешение
setting-fullscreen = Полноэкранный режим
setting-vsync = Вертикальная синхронизация
setting-quality = Качество графики
setting-world-detail = Детализация мира

# Настройки звука
setting-master-volume = Общая громкость
setting-music-volume = Громкость музыки
setting-sfx-volume = Громкость эффектов
setting-ui-volume = Громкость интерфейса
setting-ambience-volume = Громкость окружения

# Общие настройки
setting-language = Язык
setting-theme = Тема интерфейса
setting-allow-multiple-instances = Разрешить несколько копий приложения

# Управление
control-forward = Вперед
control-backward = Назад
control-left = Влево
control-right = Вправо
control-jump = Прыжок
control-sprint = Спринт
control-interact = Взаимодействие

# Значения и общее
val-on = ВКЛ
val-off = ВЫКЛ
label-version = Версия: { $version }
label-loading = Загрузка...

val-low = Низкое
val-medium = Среднее
val-high = Высокое
val-ultra = Ультра

lang-en = Английский
lang-ru = Русский

theme-dark = Тёмная
theme-light = Светлая

# Pause Menu
pause-title = Пауза
pause-resume = Продолжить
pause-settings = Настройки
pause-main-menu = Выход в главное меню
pause-exit-game = Выход из игры

# --- Логи и системные сообщения ---
# Загрузка плагинов
log-boot-init = [BootPlugin] Инициализация...
log-boot-complete = [BootPlugin] Загрузка завершена. Переход...

# Локализация
log-loc-setup = [Localization] Настройка Fluent для локали: { $locale }
log-loc-resolved = [Localization] Разрешена запрошенная локаль '{ $requested }' -> '{ $resolved }'
log-loc-missing-dir = [Localization] Директория локалей отсутствует в assets ({ $path }); пропуск загрузки и использование en-US
log-loc-applying = [Localization] Применение смены языка: { $locale }
log-loc-updated = [Localization] Ресурс локализации обновлен на { $locale }
log-loc-updating-ui = [Localization] Обновление UI текстов для нового языка...
log-loc-missing-key = [Localization] Ключ отсутствует во всех бандлах: { $key }
log-loc-format-error = [Localization] Ошибка форматирования ({ $bundle }) для '{ $key }': { $error }

# Загрузка ресурсов (UI)
log-loading-reset = [LoadingUI] Сброс трекера загрузки.
log-loading-spawn = [LoadingUI] Создание экрана загрузки...
log-loading-failed = [LoadingUI] Не удалось загрузить ассет: { $asset }
log-loading-complete = [LoadingUI] Контент загружен. Переход к InGame.
log-loading-cleanup = [LoadingUI] Очистка экрана загрузки.

ui-loading-title = ЗАГРУЗКА КОНТЕНТА
ui-loading-init = Инициализация...

# Советы на экране загрузки
hint-scan-clusters = Сканирование локальных звездных скоплений...
hint-calibrate-gravity = Калибровка гравитационных моделей планет...
hint-warm-reactors = Разогрев реакторов термоядерного синтеза...
hint-sync-trajectories = Синхронизация орбитальных траекторий...
hint-opt-nav = Оптимизация навигации на сверхсветовых скоростях...

# Инфо о прогрессе загрузки
info-loading-engine = Инициализация движка...
info-loading-stars = Загрузка звездных каталогов...
info-loading-textures = Синтез текстур планет...
info-loading-models = Создание атмосферных моделей...
info-loading-finalizing = Финализация состояния мира...

# Настройки
log-settings-switch-tab = [Settings] Переключение на вкладку: { $tab }

# Игра
log-game-init = [Game] Инициализация 3D мира игры...
log-game-sphere-fail = [Game] Не удалось создать меш сферы; пропуск планеты.
log-game-enjoy = [Game] Передача управления завершена. Наслаждайтесь Космосом!
