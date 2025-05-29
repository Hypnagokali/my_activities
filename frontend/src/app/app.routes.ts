import { Routes } from '@angular/router';
import { AccountComponent } from './account/account.component';
import { LoginComponent } from './login/login.component';
import { loginGuard } from './core/guards/login.guard';
import { authGuard } from './core/guards/auth.guard';
import { RegisterTotpComponent } from './register-totp/register-totp.component';

export const routes: Routes = [
    { 
        path: 'login', 
        component: LoginComponent,
        canActivate: [loginGuard]
    },
    { 
        path: 'account',
        component: AccountComponent,
        canActivate: [authGuard]
    },
    { 
        path: 'register-totp',
        component: RegisterTotpComponent,
        canActivate: [authGuard]
    }
];
