import { Injectable } from '@angular/core';
import { BehaviorSubject, catchError, map, Observable, of, take } from 'rxjs';
import { HttpClient } from '@angular/common/http';
import { Router } from '@angular/router';
import { User } from '../models/user.model';

@Injectable({
  providedIn: 'root'
})
export class AuthService {

  private _userSubject: BehaviorSubject<User | null> = new BehaviorSubject<User | null>(null);

  public user: Observable<User | null> = this._userSubject.asObservable();

  constructor(private http: HttpClient, private router: Router) {
    console.log('Construct AuthService')
    this.retrieveUser();
  }

  retrieveUser() {
    console.log('retrieveUser()');
    this.http.get<User>("/api/current-user")
      .subscribe(data => {
        console.log('Successfully retrieved user data');
        this._userSubject.next(data);
      })
  }

  getUser(): User | null {
    return this._userSubject.getValue();
  }

  canActivateLogin(): Observable<boolean> {
    return this.http.get<User>("/api/current-user")
    .pipe(
      catchError(() => {
        return of(null)
      }),
      take(1),
      map(user => {
        if (user) {
          this.router.navigate(['/account'])
          return false;
        }

        return true;
      })
    );
  }

  canActivateAuthenticated(): Observable<boolean> {
    return this.http.get<User>("/api/current-user")
    .pipe(
      catchError(() => {
        return of(null)
      }),
      take(1),
      map(user => {
        if (!user) {
          this.router.navigate(['/login'])
          return false;
        }

        return true;
      })
    );
  }

}
