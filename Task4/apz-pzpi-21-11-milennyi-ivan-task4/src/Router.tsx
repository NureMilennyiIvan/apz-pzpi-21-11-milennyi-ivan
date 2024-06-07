import { useEffect, useState } from 'react'
import { UserRole } from './utils/UserRole'
import { AuthUser } from './utils/AuthUser'
import { getAuthUserFromLocalStorage, saveAuthUserToLocalStorage } from './utils/helpers'
import { useTranslation } from 'react-i18next'
import { BrowserRouter, Route, Routes } from 'react-router-dom'
import styles from './App.module.css'
import App from './App'
import { SheepDetailsPage } from './components/shepherd/SheepDetailsPage'
import { ShearForm } from './components/shepherd/ShearForm'
import { FeedPage } from './components/storekeeper/FeedPage'

const Router = () => {
   const [user, setUser] = useState<AuthUser>(new AuthUser(1, UserRole.Storekeeper));
   //const [user, setUser] = useState<AuthUser>(new AuthUser(null, UserRole.Unauthorized));
   const logout = () =>{
       const unauthorized_user = new AuthUser(null, UserRole.Unauthorized);
       saveAuthUserToLocalStorage("user", unauthorized_user);
       setUser(unauthorized_user);
   }
   useEffect(() => {
       const result = getAuthUserFromLocalStorage("user");
       if (result.Id != null && user.Id == null){
           setUser(new AuthUser(result.Id, result.Role));
       }
   }, [user])

   const { t, i18n } = useTranslation();

   const changeLanguage = (lng?: string | undefined) => {
     i18n.changeLanguage(lng);
   };

   return (
   <div className={styles.container}>

      <BrowserRouter>
         <header className={styles.header}>
           <button className={styles.button} style={{minWidth: "200px"}} onClick={() => changeLanguage(i18n.language === 'en' ? 'ua' : 'en')}>{t('router.switchButtonText')}</button>
           {user.Role != UserRole.Unauthorized && (
               <div className={styles.buttonContainer}>
                   <button className={styles.button} onClick={logout}>{t('router.logoutButtonText')}</button>
               </div>
           )}
        </header>
        <Routes>
          <Route element={<App user={user} setUser={setUser}/>} path="/"></Route>
          <Route element={<SheepDetailsPage user={user} setUser={setUser}/>} path="/sheep/:sheepId/details"></Route>
          <Route element={<FeedPage user={user} setUser={setUser}/>} path="/feed/:feedId/details"></Route>
          <Route element={<ShearForm user={user} setUser={setUser}/>} path="/sheep/:sheepId/create/shearing-log"></Route>
          <Route element={<div>Not Found 404</div>} path="*"></Route>
        </Routes>
      </BrowserRouter>
      <footer className={styles.footer}>

      </footer>
   </div>)
}

export default Router;
