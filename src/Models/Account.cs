using System.ComponentModel.DataAnnotations;

namespace BetterMoneyManager.Models;

public class Account
{
    public int Id { get; set; }

    [Required(AllowEmptyStrings = false)]
    [StringLength(40, MinimumLength = 1)]
    public string Name { get; set; }

    [Required(AllowEmptyStrings = false)]
    [StringLength(40, MinimumLength = 1)]
    public string BankName { get; set; }

    [Required]
    public AccountType AccountType { get; set; }

    [Required]
    public decimal Balance { get; set; }

    [Required]
    public decimal AvailableBalance { get; set; }

    public bool Favorite { get; set; }

    public DateTime DateCreated { get; set; }
}