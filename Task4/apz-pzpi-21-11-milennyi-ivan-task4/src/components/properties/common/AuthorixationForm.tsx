import { useState } from "react";
import { ShepherdService } from "../../../api/services/ShepherdService";
import { StorekeeperService } from "../../../api/services/StorekeeperService";
import { UserRole } from "../../../utils/UserRole";
import { IUserProps } from "../IUserProps";
import { useTranslation } from "react-i18next";
import { handleElementChange, hashPassword, saveAuthUserToLocalStorage } from "../../../utils/helpers";
import { AuthUser } from "../../../utils/AuthUser";
import styles from "./AuthorizationForm.module.css";

const AuthorizationForm: React.FC<IUserProps> = ({ setUser }) => {
    const shepherdService = new ShepherdService();
    const storekeeperService = new StorekeeperService();
    const [username, setUsername] = useState('');
    const [password, setPassword] = useState('');
    const [role, setRole] = useState<UserRole>(UserRole.Shepherd);

    const [errorUsername, setErrorUsername] = useState('');
    const [errorPassword, setErrorPassword] = useState('');

    const {t} = useTranslation();

    const signIn = async () =>{
        if (username.length < 1) {
            setErrorUsername("authorizationForm.errorUsernameHeader1");
            return;
        } else {
            setErrorUsername('');
        }
        if (password.length < 1) {
            setErrorPassword("authorizationForm.errorPasswordHeader");
            return;
        } else {
            setErrorPassword('');
        }
        if (role === UserRole.Shepherd){
            try{
                const hashedPassword = await hashPassword(password);
                const authorizedShepherd = await shepherdService.authorize(username, hashedPassword);
                if (authorizedShepherd){
                    let user = new AuthUser(authorizedShepherd.id, UserRole.Shepherd);
                    saveAuthUserToLocalStorage("user", user);
                    setUser(user);
                    return;
                }
                throw new Error();
            }
            catch (error){
                setErrorUsername("authorizationForm.errorUsernameHeader2");
            }
        }
        else if (role === UserRole.Storekeeper){
            try{
                const hashedPassword = await hashPassword(password);
                const authorizedStorekeeper = await storekeeperService.authorize(username, hashedPassword);
                if (authorizedStorekeeper){
                    let user = new AuthUser(authorizedStorekeeper.id, UserRole.Storekeeper);
                    saveAuthUserToLocalStorage("user", user);
                    setUser(user);
                    return;
                }
                throw new Error();
            }
            catch (error){
                setErrorUsername("authorizationForm.errorUsernameHeader2");
            }
        }
    }

    return <div className={styles.body}>

      <div className={styles.form}>

        <div>
            <label className={styles.label}>{t("authorizationForm.usernameHeader")}</label>
            <input className={styles.input} type="usernamr" placeholder={t("authorizationForm.usernamePlaceholder")} value={username} onChange={((e) => handleElementChange(e, setUsername))}/>
            {errorUsername && <span className={styles.error}>{t(errorUsername)}</span>}
        </div>
        <div>
            <label className={styles.label}>{t("authorizationForm.passwordHeader")}</label>
            <input className={styles.input} type="password" placeholder={t("authorizationForm.passwordPlaceholder")} value={password} onChange={((e) => handleElementChange(e, setPassword))}/>     
            {errorPassword && <span className={styles.error}>{t(errorPassword)}</span>}       
        </div>
        <div className={styles.radioButtons}>
            <label className={styles.label}>{t("authorizationForm.shepherdStatus")}</label>
            <label className={styles.radio}>
                <input className={styles.radioInput} type="radio" name="option" defaultChecked={true} value={UserRole.Shepherd} onChange={((e) => handleElementChange(e, setRole))}/>
                <span className={styles.checkmark}></span>
            </label>
            <label className={styles.label}>{t("authorizationForm.storekeeperStatus")}</label>
            <label className={styles.radio}>
                <input className={styles.radioInput} type="radio" name="option" value={UserRole.Storekeeper} onChange={((e) => handleElementChange(e, setRole))}/>
                <span className={styles.checkmark}></span>
            </label>
        </div>
        <div className={styles.actionButtonsContainer}>
            <button className={styles.actionButton} onClick={signIn}>{t("authorizationForm.signInButtonText")}</button>
        </div>
      </div>
    </div>
}

export default AuthorizationForm;