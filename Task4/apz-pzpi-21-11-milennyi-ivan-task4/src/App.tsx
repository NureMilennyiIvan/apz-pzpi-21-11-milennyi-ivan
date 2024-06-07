import { useNavigate } from 'react-router-dom';
import styles from './App.module.css';
import { useEffectUser } from './utils/helpers';
import { IUserProps } from './components/properties/IUserProps';
import { UserRole } from './utils/UserRole';
import AuthorizationForm from './components/common/AuthorixationForm';
import { ShepherdMainPage } from './components/shepherd/ShepherdMainPage';
import { StorekeeperMainPage } from './components/storekeeper/StorekeeperMainPage';

const App: React.FC<IUserProps> = ({user, setUser}) => {
  const navigate = useNavigate();
  useEffectUser(user, navigate);

  return (
      <div className={styles.content}>
        {user.Role === UserRole.Unauthorized && (
          <AuthorizationForm user={user} setUser={setUser}/>
        )}
        {user.Role === UserRole.Shepherd && (
          <ShepherdMainPage user={user} setUser={setUser}/>
        )}
        {user.Role === UserRole.Storekeeper && (
          <StorekeeperMainPage user={user} setUser={setUser}/>
        )}
        {user.Role === UserRole.Admin && (
          <div></div>
          //<AdminMainPage/>
        )}        
      </div>
  )
}

export default App
