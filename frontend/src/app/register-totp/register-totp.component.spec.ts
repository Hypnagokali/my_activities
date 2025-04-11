import { ComponentFixture, TestBed } from '@angular/core/testing';

import { RegisterTotpComponent } from './register-totp.component';

describe('RegisterQrcodeComponent', () => {
  let component: RegisterTotpComponent;
  let fixture: ComponentFixture<RegisterTotpComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [RegisterTotpComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(RegisterTotpComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
