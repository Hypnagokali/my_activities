import { Component } from '@angular/core';
import { AuthService } from '../app/services/auth.service';
import { Observable } from 'rxjs';
import { User } from '../app/models/user.model';
import { CommonModule } from '@angular/common';

@Component({
  selector: 'app-account',
  imports: [CommonModule],
  templateUrl: './account.component.html',
  styleUrl: './account.component.css'
})
export class AccountComponent {

  user: Observable<User | null>;

  constructor(private authService: AuthService) {
    this.user = this.authService.user;
  }

}
