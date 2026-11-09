# 🚀 CI/CD Pipelines

Учебный набор независимых проектов, демонстрирующих CI/CD на GitHub Actions: автоматические проверки, тестирование, сборку приложений и доставку готовых артефактов.

Каждый сценарий находится в **отдельном GitHub-репозитории**. Этот репозиторий служит главным каталогом и точкой входа.

## 📋 Список проектов

| # | Проект | Стек | Что делает | Артефакт / публикация | Триггер |
|:-:|---|---|---|---|---|
| 1 | [hello-rust](https://github.com/anitouchq/hello-rust) | 🦀 Rust | fmt, Clippy, tests, Docker build | Docker image → GHCR | push в `main` |
| 2 | [hello-go](https://github.com/anitouchq/hello-go) | 🐹 Go | gofmt, vet, tests, Docker build | Docker image → GHCR | push в `main` |
| 3 | [hello-go-releases](https://github.com/anitouchq/hello-go-releases) | 🐹 Go | lint, tests, matrix build | Бинарники для 5 платформ → GitHub Releases | тег `v*` |
| 4 | [hello-python](https://github.com/anitouchq/hello-python) | 🐍 Python | Ruff, pytest, PyInstaller | Бинарники для 3 платформ → GitHub Releases | тег `v*` |
| 5 | [hello-static](https://github.com/anitouchq/hello-static) | 🌐 HTML/CSS/JS | HTML, CSS и JS validation | Static site → GitHub Pages | push в `main` |
| 6 | [hello-pages](https://github.com/anitouchq/hello-pages) | ⚛️ React + Vite | ESLint, production build | `dist/` → GitHub Pages | push в `main` |

> Ссылки ведут на отдельные репозитории проектов. Публикация и статусы будут отмечены после создания репозиториев и успешных запусков workflow.

## 🧩 Архитектура

Каждый проект автономен и содержит собственные:
- исходный код и конфигурацию;
- тесты и проверки качества;
- Dockerfile — для проектов с контейнеризацией;
- workflow в `.github/workflows/`;
- README с командами запуска и описанием CI/CD.

Главный репозиторий не собирает приложения и не выполняет деплой. Он объединяет проекты в единое портфолио.

## 🐳 Docker → GitHub Container Registry

Проекты Rust и Go демонстрируют последовательность:

1. Проверка форматирования и статический анализ.
2. Запуск тестов.
3. Сборка приложения.
4. Сборка Docker-образа.
5. Публикация образа в GHCR после успешного CI.

Пример запуска опубликованного образа:

```bash
docker pull ghcr.io/anitouchq/hello-rust:latest
docker run --rm ghcr.io/anitouchq/hello-rust:latest
```

## 📦 GitHub Releases

Go- и Python-проекты собирают исполняемые файлы для нескольких операционных систем. Новый релиз запускается тегом:

```bash
git tag v1.0.0
git push origin v1.0.0
```

GitHub Actions проверяет исходный код, выполняет матричную сборку и прикладывает бинарники к GitHub Release.

## 🌐 GitHub Pages

Два отдельных сценария доставки сайта:

- **Static HTML** — статические файлы валидируются и публикуются без этапа сборки.
- **React + Vite** — CI запускает линтер и production build; собранный `dist/` передаётся в job деплоя как artifact.

## 🔐 Безопасность и permissions

В workflow используются минимально необходимые разрешения:
- `contents: read` — получение исходного кода;
- `packages: write` — публикация Docker-образов в GHCR;
- `contents: write` — создание GitHub Releases;
- `pages: write` и `id-token: write` — деплой в GitHub Pages.

Для GHCR и GitHub Releases применяется встроенный `GITHUB_TOKEN`. Не следует добавлять токены непосредственно в исходный код.

## 🛠 Технологии

![GitHub Actions](https://img.shields.io/badge/GitHub%20Actions-CI%2FCD-2088FF?logo=githubactions&logoColor=white)
![Docker](https://img.shields.io/badge/Docker-Containerization-2496ED?logo=docker&logoColor=white)
![GHCR](https://img.shields.io/badge/GHCR-Container%20Registry-181717?logo=github&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-stable-000000?logo=rust&logoColor=white)
![Go](https://img.shields.io/badge/Go-1.23-00ADD8?logo=go&logoColor=white)
![Python](https://img.shields.io/badge/Python-3.12-3776AB?logo=python&logoColor=white)

---

Учебное портфолио · GitHub Actions · CI/CD
