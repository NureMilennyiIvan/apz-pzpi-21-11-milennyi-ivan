import ReactDOM from 'react-dom/client'
import './index.css'
import i18n from 'i18next';
import { initReactI18next } from 'react-i18next'

import translationEN from './locales/en.json';
import translationUA from './locales/ua.json';
import Router from './Router';

const resources = {
  en: {
    translation: translationEN,
  },
  ua: {
    translation: translationUA,
  },
};

i18n
  .use(initReactI18next)
  .init({
    resources,
    lng: 'ua',
    fallbackLng: 'ua',
    interpolation: {
      escapeValue: false,
    },
  });

ReactDOM.createRoot(document.getElementById('root')!).render(
    <Router/>
)
