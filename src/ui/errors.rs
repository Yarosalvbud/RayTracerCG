use thiserror::Error;

#[derive(Error, Debug, Clone, Copy)]
pub enum UiError{
    #[error("Невозможно прочитать данные перемещения объекта, каждая компонента - вещественное число")]
    MoveDataFormatError,
    #[error("Невозможно прочитать данные о uv развертке")]
    BadUvError,
    #[error("Невозможно прочитать данные из stl файла")]
    BadStlError,
    #[error("Невозможно добавить объект, так как не указан путь к нему")]
    NoPathError,
    #[error("Данного объекта не существует")]
    ObjectNotFoundError,
    #[error("Требуется указать номер объекта - целое положительное число")]
    NumberNotFoundError,
    #[error("Для того, чтобы применить текстуру к объекту требуется UV развертка")]
    NoUnwrapError,
    #[error("Не указана текстура")]
    NoTextureError,
    #[error("Ошибка при загрузке текстуры")]
    LoadTextureError,
    #[error("Ошибка при загрузке карты нормалей")]
    LoadNormalsError,
    #[error("Некорректная uv развертка")]
    BadUVError,
    #[error("Область видимости камеры задана неверно, область видимости - вещественное положительное число меньше 180.0")]
    BadFovError,
    #[error("Интенсивность света должна быть в диапазоне от 0.0 до 1.0")]
    LightIntensityError,
    #[error("Неправильное значение для цвета, цвет в формате rgb, каждая компонента в диапазоне от 0 до 255")]
    ColorError,
}